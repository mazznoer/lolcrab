use std::io;
use std::io::prelude::*;
use termcolor::{ColorSpec, WriteColor};

pub struct AnsiEscaper<T: Write + WriteColor> {
    writer: T,
    is_escaping: bool,
}

impl<T: Write + WriteColor> AnsiEscaper<T> {
    pub fn new(writer: T) -> AnsiEscaper<T> {
        AnsiEscaper {
            writer,
            is_escaping: false,
        }
    }
}

impl<T: Write + WriteColor> io::Write for AnsiEscaper<T> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        for _byte in buffer {
            let byte = *_byte;
            if byte == 0x1B {
                self.is_escaping = true;
            } else if self.is_escaping && (b'a' <= byte && byte <= b'z')
                || (b'A' <= byte && byte <= b'Z')
            {
                self.is_escaping = false
            }
        }
        self.writer.write(buffer)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<T: Write + WriteColor> WriteColor for AnsiEscaper<T> {
    fn supports_color(&self) -> bool {
        self.writer.supports_color()
    }
    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        if !self.is_escaping {
            self.writer.set_color(spec)?;
        }
        Ok(())
    }
    fn reset(&mut self) -> io::Result<()> {
        self.writer.reset()
    }
}
