mod args;

use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, Result};
use log::{info, trace};
use env_logger::Env;
use args::Args;
use clap::Parser;
use std::thread;
use itertools::Itertools;

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

        match command_parts.next().unwrap() {
            "echo" => {
                let response = format!("{}\n", command_parts.join(" "));
                stream.write_all(response.as_bytes())?;
                stream.flush()?;
                trace!("Sent {} to {}", request, peer_addr);
            }
            _ => {
                stream.write_all("Command not understood\n".as_bytes())?;
                stream.flush()?;
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
