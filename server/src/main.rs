use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, Result};
use log::info;
use env_logger::Env;

use bufstream::BufStream;

fn handle_client(conn: TcpStream) -> Result<()> {
    let mut request = String::with_capacity(512);
    let mut stream = BufStream::new(&conn);

    loop {
        // Send prompt to client
        stream.write_all(b"> ")?;
        stream.flush()?;

        let request_size = stream.read_line(&mut request)?;
        
        if request_size == 0 {
            break;
        }

        // Just echo the request to the client
        stream.write_all(request.as_bytes())?;
        stream.flush()?;
    }

    info!("Dropping connection from {}", conn.peer_addr()?.to_string());

    Ok(())
}

// TODO: Multithreaded
// TODO: Port from clap
// TODO: It's returning all previous messages. Fix it.
fn main() -> Result<()> {
    env_logger::Builder
              ::from_env(Env::default().default_filter_or("info"))
              .init();

    let listener = TcpListener::bind("0.0.0.0:3333")?;

    for conn in listener.incoming() {
        let conn = conn?;
        info!("Received connection from {}", conn.peer_addr()?.to_string());
        handle_client(conn)?;
    }

    Ok(())
}
