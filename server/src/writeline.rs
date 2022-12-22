use bufstream::BufStream;
use std::io::{Result, Write, Read};

pub trait WriteLine {
   fn write_line(&mut self, line: &str) -> Result<()>;
}

impl<T: Write + Read> WriteLine for BufStream<T> {
    fn write_line(&mut self, line: &str) -> Result<()> {
       self.write_all(line.as_bytes())?;
       self.write("\n".as_bytes())?;
       self.flush()?;
       Ok(())
    }
}
