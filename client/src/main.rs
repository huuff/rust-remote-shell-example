mod args;

use crate::args::Args;
use env_logger::Env;
use bufstream::BufStream;
use clap::Parser;
use native_tls::TlsConnector;
use std::{io::{self, Read, Write}, net::TcpStream};
use crlf::{ReadCrlfLine, WriteCrlfLine};
use log::trace;
use std::error::Error;

pub trait ReadWrite: Read + Write {}
impl <T: Read + Write> ReadWrite for T {}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder
        ::from_env(Env::default().default_filter_or("info"))
        .init();

    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);

    let conn: Box<dyn ReadWrite> = if args.tls {
        let connector = TlsConnector::builder()
                                    .danger_accept_invalid_certs(true)
                                    .build()?;
        let conn = TcpStream::connect(&addr)?;
        Box::new(connector.connect(&addr, conn)?)
    } else {
        Box::new(TcpStream::connect(addr)?)
    };
    let mut stream = BufStream::new(conn);
    let mut request = String::with_capacity(512);
    let mut response = String::with_capacity(4096);
    let mut prompt_buffer = [0u8; 2];

    loop {
        response.clear();
        let read_bytes = stream.read_crlf_line(&mut response)?;
        trace!("Received {} bytes from server", read_bytes);
        println!("{}", response);

        if request.trim() == "exit" {
            break;
        }

        stream.read_exact(&mut prompt_buffer)?;
        print!("{}", std::str::from_utf8(&mut prompt_buffer).unwrap());
        io::stdout().flush()?;
        stream.flush()?;
        request.clear();
        trace!("Requesting input from user");
        io::stdin().read_line(&mut request)?;
        stream.write_crlf_line(request.trim().as_bytes())?;
        stream.flush()?;
        trace!("Sent {} to the server", request.trim());
    }

    Ok(())
}
