// XXX: Literal copy-paste of my rust crlf library

use std::io::{Read, Write, BufRead};
use std::str;
use std::error::Error;

use bufstream::BufStream;

pub trait ReadCrlfLine {
    fn read_crlf_line(&mut self, buffer: &mut String) -> Result<usize, Box<dyn Error>>;
}

impl<T: Read + Write> ReadCrlfLine for BufStream<T> {
    fn read_crlf_line(&mut self, buffer: &mut String) -> Result<usize, Box<dyn Error>> {
        let fill_buf = self.fill_buf()?;

        if fill_buf.is_empty() {
            return Ok(0);
        }

        let mut consumed = 0;
        // XXX: Spaghetti conditional
        while consumed < fill_buf.len() // Buffer is not finished
            && (consumed == fill_buf.len()-1 // If it's the end of the buffer, just finish reading
                                             // it
            || (fill_buf[consumed] != b'\r' || fill_buf[consumed+1] != b'\n')) // Ensure CRLF
                                                                               // hasn't been found
                                                                               // yet 
        {
            consumed += 1; 
        }

        buffer.push_str(str::from_utf8(&fill_buf[..consumed])?);

        if consumed < fill_buf.len() {
            // Found a CRLF before the buffer ends, so we manually consume it
            consumed += 2;
        }
        self.consume(consumed);

        return Ok(consumed);
    }
}

pub trait WriteCrlfLine {
    fn write_crlf_line(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>>;
}

impl<T: Write + Read> WriteCrlfLine for BufStream<T> {
    fn write_crlf_line(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
       self.write_all(buf)?;
       self.write(b"\r\n")?;
       self.flush()?;
       Ok(())
    } 
}

