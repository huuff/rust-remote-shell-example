mod args;
mod writeline;

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

        let mut command_parts = request.split_whitespace();

        // TODO: Logging the command that is running
        match command_parts.next().unwrap() {
            "echo" => {
                // TODO: Correct word splitting?
                stream.write_line(command_parts.join(" ").as_str())?;
                trace!("Sent {} to {}", request, peer_addr);
            },
            "ls" => {
                // TODO: Arguments?
                let dirs = fs::read_dir(".")?.map(|f| f.unwrap().path().display().to_string()).join("\n");
                stream.write_line(dirs.as_str())?;
            },
            "cat" => {
                // TODO: Handle error (missing file?)
                let argument = command_parts.next().expect("Missing an argument to cat");
                let file = File::open(argument)?;
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    stream.write_line(line?.as_str())?;
                }
            },
            "cd" => {
                let argument = command_parts.next().expect("Missing an argument to cd");
                env::set_current_dir(argument)?;
            },
            "exit" => {
                stream.write_line("Bye")?;
                break;
            },
            _ => {
                stream.write_line("Command not understood")?;
            },
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
