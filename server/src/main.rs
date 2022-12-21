use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, Result};

use bufstream::BufStream;

fn handle_client(conn: TcpStream) -> Result<()> {
    let mut request = String::with_capacity(512);
    let mut stream = BufStream::new(&conn);

    // Send prompt to client
    stream.write_all(b"> ")?;
    stream.flush()?;

    loop {
        let request_size = stream.read_line(&mut request)?;
        
        if request_size == 0 {
            break;
        }

        // Just echo the request to the client
        stream.write_all(request.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333")?;

    for conn in listener.incoming() {
        handle_client(conn?)?;
    }

    Ok(())
}
