mod args;
mod crlf;
mod command;

use std::env;
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, Result, BufReader, Read};
use lazy_static::lazy_static;
use log::{info, trace};
use env_logger::Env;
use args::Args;
use clap::Parser;
use uuid::Uuid;
use std::{thread, fs};
use itertools::Itertools;
use crate::crlf::WriteCrlfLine;
use bufstream::BufStream;
use crate::command::Command;

fn handle_client(conn: TcpStream, password: &str) -> Result<()> {
    let peer_addr = conn.peer_addr()?.to_string();
    let mut request = String::with_capacity(512);
    let mut stream = BufStream::new(&conn);

    let mut password_guesses = 0;

    stream.write_crlf_line("Please provide the password".as_bytes())?;
    loop {
        stream.write_all(b"> ")?;
        stream.flush()?;

        request.clear();

        stream.read_line(&mut request)?;
        trace!("Received password attempt {}", request.trim());

        if request.trim() != password {
            stream.write_crlf_line("Incorrect password".as_bytes())?;
            password_guesses += 1;
            
            if password_guesses == 3 {
                stream.write_crlf_line("Too many incorrect guesses. Kicking you out".as_bytes())?;
                return Ok(());
            }
        } else {
            trace!("Client {} sent the correct password. Accepted", peer_addr);
            break;
        }
    }

    // TODO: A "help" command?
    stream.write_crlf_line("Enter your command".as_bytes())?;
    loop {
        // Send prompt to client
        stream.write_all(b"> ")?;
        stream.flush()?;
        trace!("Sent prompt to {}", peer_addr);

        request.clear();

        let request_size = stream.read_line(&mut request)?;
        trace!("Received `{}` from {}", request.trim(), peer_addr);

        if request_size == 0 {
            break;
        }

        let command = Command::parse(&request);

        if let Err(err) = command {
            stream.write_crlf_line(err.msg.as_bytes())?;
            continue;
        }

        // TODO: Log responses? how?
        match command.unwrap() {
            Command::Echo(echo) => {
                stream.write_crlf_line(echo.message.as_bytes())?;
            },
            Command::Ls(ls) => {
                let target_dir = ls.target_directory.unwrap_or(String::from("."));
                let read_dir_result = fs::read_dir(target_dir);

                match read_dir_result {
                    Ok(dir_contents) => {
                        let dirs = dir_contents.map(|f| f.unwrap().path().display().to_string()).join("\n");
                        trace!("Sending {} to {}", dirs, peer_addr);
                        stream.write_crlf_line(dirs.as_bytes())?;
                    },
                    Err(err) => {
                        stream.write_crlf_line(err.to_string().as_bytes())?;
                    }
                }
            },
            Command::Cd(cd) => {
                env::set_current_dir(cd.target_directory)?;
            },
            Command::Cat(cat) => {
                let file_open_result = File::open(cat.file);

                match file_open_result {
                    Ok(file) => {
                        let mut buf = Vec::new();
                        let mut reader = BufReader::new(file);
                        reader.read_to_end(&mut buf)?;
                        stream.write_crlf_line(&buf)?;
                    },
                    Err(err) => {
                        stream.write_crlf_line(err.to_string().as_bytes())?;
                    }
                }

            },
            Command::Exit(_) => {
                stream.write_crlf_line("Bye".as_bytes())?;
                break;
            },
            Command::Pwd(_) => {
                let current_dir = env::current_dir()?;
                stream.write_crlf_line(current_dir.to_str().unwrap().as_bytes())?;
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

    lazy_static!(
        pub static ref PASSWORD: String = Uuid::new_v4().to_string();  
        );

    println!("The password is {}", PASSWORD.as_str());

    loop {
        for conn in listener.incoming() {
            let conn = conn?;
            info!("Received connection from {}", conn.peer_addr()?.to_string());
            let _handle = thread::spawn(|| { handle_client(conn, PASSWORD.as_str()).unwrap() });
        }

    }

}
