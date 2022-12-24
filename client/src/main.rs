mod args;
mod writeline;

use crate::args::Args;
use env_logger::Env;
use bufstream::BufStream;
use clap::Parser;
use std::{io::{Result, BufRead, self, Read, Write}, net::TcpStream};
use writeline::WriteLine;
use log::trace;

fn main() -> Result<()> {
    env_logger::Builder
        ::from_env(Env::default().default_filter_or("info"))
        .init();

    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let conn = TcpStream::connect(addr)?;
    let mut stream = BufStream::new(&conn);
    let mut request = String::with_capacity(512);
    let mut response = String::with_capacity(4096);
    let mut prompt_buffer = [0u8; 2];

    loop {
        response.clear();
        stream.read_line(&mut response)?;
        print!("{}", response);
        stream.read_exact(&mut prompt_buffer)?;
        print!("{}", std::str::from_utf8(&mut prompt_buffer).unwrap());
        io::stdout().flush()?;
        stream.flush()?;
        request.clear();
        trace!("Requesting input from user");
        io::stdin().read_line(&mut request)?;
        stream.write_line(request.trim())?;
        stream.flush()?;
        trace!("Sent {} to the server", request);
    }

}
