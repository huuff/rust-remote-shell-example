mod args;
mod writeline;

use crate::args::Args;
use bufstream::BufStream;
use clap::Parser;
use std::{io::{Result, BufRead, self}, net::TcpStream};
use writeline::WriteLine;

fn main() -> Result<()> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let conn = TcpStream::connect(addr)?;
    let mut stream = BufStream::new(&conn);
    let mut request = String::with_capacity(512);
    let mut response = String::with_capacity(4096);

    loop {
        response.clear();
        stream.read_line(&mut response)?;
        println!("{}", response);
        request.clear();
        io::stdin().read_line(&mut request)?;
        stream.write_line(request.as_str())?;
    }

}
