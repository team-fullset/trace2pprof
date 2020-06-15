use std::error::Error;
use std::io::{BufRead, BufReader, Read};

pub struct TraceFileReader<R> {
    reader: BufReader<R>,
    buffer: String,
}

impl<R: Read> TraceFileReader<R> {
    pub fn new(r: R) -> Self {
        Self {
            reader: BufReader::new(r),
            buffer: String::new(),
        }
    }

    pub fn read_line(&mut self) -> Result<Option<(u64, &str)>, Box<dyn Error>> {
        self.buffer.clear();

        let n = self.reader.read_line(&mut self.buffer)?;
        if n == 0 {
            return Ok(None);
        }

        let mut parts = self.buffer.trim().split(": ");

        let instr_addr = u64::from_str_radix(parts.next().unwrap(), 16)?;
        let instr_asm = parts.next().unwrap();

        Ok(Some((instr_addr, instr_asm)))
    }
}
