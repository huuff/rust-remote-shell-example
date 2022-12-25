mod args;
mod command;

use std::env;
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, BufReader, Read};
use std::sync::Arc;
use log::{info, trace};
use env_logger::Env;
use args::Args;
use clap::Parser;
use native_tls::{Identity, TlsAcceptor};
use std::{thread, fs};
use itertools::Itertools;
use crlf::WriteCrlfLine;
use bufstream::BufStream;
use crate::command::Command;
use std::error::Error;

pub trait ReadWrite: Read + Write {}
impl <T: Read + Write> ReadWrite for T {}

fn handle_client(conn: Box<dyn ReadWrite>, password: &str) -> Result<(), Box<dyn Error>> {
    //let peer_addr = conn.peer_addr()?.to_string();
    let peer_addr = String::from("Gotta fix this (previous comment)");
    let mut request = String::with_capacity(512);
    let mut stream = BufStream::new(conn);

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

    stream.write_crlf_line("Enter your command".as_bytes())?;
    let mut response: Option<String>;
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

        match command.unwrap() {
            Command::Echo(echo) => {
                response = Some(echo.message);
            },
            Command::Ls(ls) => {
                let target_dir = ls.target_directory.unwrap_or(String::from("."));
                let read_dir_result = fs::read_dir(target_dir);

                match read_dir_result {
                    Ok(dir_contents) => {
                        let dirs = dir_contents.map(|f| f.unwrap().path().display().to_string()).join("\n");
                        trace!("Sending {} to {}", dirs, peer_addr);
                        response = Some(dirs);
                    },
                    Err(err) => {
                        response = Some(err.to_string());
                    }
                }
            },
            Command::Cd(cd) => {
                env::set_current_dir(&cd.target_directory)?;
                response = Some(format!("Changed directory to {}", cd.target_directory));
            },
            Command::Cat(cat) => {
                let file_open_result = File::open(cat.file);

                match file_open_result {
                    Ok(file) => {
                        let mut buf = Vec::new();
                        let mut reader = BufReader::new(file);
                        reader.read_to_end(&mut buf)?;
                        response = Some(String::from_utf8(buf).unwrap());
                    },
                    Err(err) => {
                        response = Some(err.to_string());
                    }
                }

            },
            Command::Exit(_) => {
                stream.write_crlf_line("Bye".as_bytes())?;
                break;
            },
            Command::Pwd(_) => {
                let current_dir = env::current_dir()?;
                response = Some(String::from(current_dir.to_str().unwrap()));
            },
            Command::Help(_) => {
                response = Some(String::from(r#"
    pwd - Print current directory
    cd - Change directory
    ls [dir] - List contents of [dir], or current directory
    cat file - Print contents of file
    echo [msg] - Print msg
    exit - Leave shell
    help - Print this helpful message
"#));
            },
        }

        if let Some(response) = response {
            stream.write_crlf_line(response.as_bytes())?;
            trace!("Sent {} to {}", response, peer_addr);
        }

    }

    info!("Dropping connection from {}", peer_addr);

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder
        ::from_env(Env::default().default_filter_or("info"))
        .init();

    let args = Args::parse();
    let bind_addr = format!("{}:{}", args.addr, args.port);
    let listener = TcpListener::bind(&bind_addr)?;

    let tls_acceptor = if let Some(cert) = args.cert {
        let mut cert = File::open(cert)?;
        let mut identity = vec![];
        cert.read_to_end(&mut identity)?;
        let identity = Identity::from_pkcs12(&identity, args.password.as_str())?;
        let acceptor = TlsAcceptor::new(identity)?;
        Some(Arc::new(acceptor))
    } else {
        None
    };

    info!("Bound to {}", bind_addr);

    println!("The password is {}", args.password.as_str());

    loop {
        for conn in listener.incoming() {
            let conn = conn?;
            info!("Received connection from {}", conn.peer_addr()?.to_string());
            
            let acceptor = tls_acceptor.as_ref().map(|a| a.clone());

            let password_clone = args.password.clone();
            let _handle = thread::spawn(move || { 
                let conn: Box<dyn ReadWrite> = if let Some(acceptor) = acceptor {
                    Box::new(acceptor.accept(conn).unwrap())
                } else {
                    Box::new(conn)
                };
                handle_client(conn, password_clone.as_str()).unwrap() 
            });
        }

    }

}
