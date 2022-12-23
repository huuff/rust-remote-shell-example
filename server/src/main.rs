mod args;
mod writeline;
mod command;

use std::env;
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, Result, BufReader};
use log::{info, trace};
use env_logger::Env;
use args::Args;
use clap::Parser;
use std::{thread, fs};
use itertools::Itertools;
use writeline::WriteLine;


use bufstream::BufStream;

use crate::command::Command;

fn handle_client(conn: TcpStream) -> Result<()> {
    let peer_addr = conn.peer_addr()?.to_string();
    let mut request = String::with_capacity(512);
    let mut stream = BufStream::new(&conn);

    loop {
        // Send prompt to client
        stream.write_all(b"> ")?;
        stream.flush()?;
        trace!("Sent prompt to {}", peer_addr);

        request.clear();

        let request_size = stream.read_line(&mut request)?;

        if request_size == 0 {
            break;
        }

        let command = Command::parse(&request);

        if let Err(err) = command {
            stream.write_line(err.msg.as_str())?;
            continue;
        }

        match command.unwrap() {
            Command::Echo(echo) => {
                stream.write_line(echo.message.as_str())?;
            },
            Command::Ls(ls) => {
                let target_dir = ls.target_directory.unwrap_or(String::from("."));
                let read_dir_result = fs::read_dir(target_dir);

                match read_dir_result {
                    Ok(dir_contents) => {
                        let dirs = dir_contents.map(|f| f.unwrap().path().display().to_string()).join("\n");
                        stream.write_line(dirs.as_str())?;
                    },
                    Err(err) => {
                        stream.write_line(err.to_string().as_str())?;
                    }
                }
            },
            Command::Cd(cd) => {
                env::set_current_dir(cd.target_directory)?;
            },
            Command::Cat(cat) => {
                // TODO: Handle error (missing file?)
                let file = File::open(cat.file)?;
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    stream.write_line(line?.as_str())?;
                }
            },
            Command::Exit(_) => {
                stream.write_line("Bye")?;
                break;
            },
            Command::Pwd(_) => {
                let current_dir = env::current_dir()?;
                stream.write_line(current_dir.to_str().unwrap())?;
            }
        }
    }

    info!("Dropping connection from {}", peer_addr);

    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder
        ::from_env(Env::default().default_filter_or("info"))
        .init();

    let args = Args::parse();
    let bind_addr = format!("{}:{}", args.addr, args.port);
    let listener = TcpListener::bind(&bind_addr)?;
    info!("Bound to {}", bind_addr);

    loop {
        for conn in listener.incoming() {
            let conn = conn?;
            info!("Received connection from {}", conn.peer_addr()?.to_string());
            let _handle = thread::spawn(|| { handle_client(conn).unwrap() });
        }

    }

}
