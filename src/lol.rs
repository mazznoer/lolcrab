use rand::random;
use std::f64::consts::PI;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;
use termcolor::{Color, ColorSpec, WriteColor};

#[derive(StructOpt)]
pub struct LolOpts {
    /// Set speed
    #[structopt(short = "s", long = "seed")]
    seed: Option<u8>,
    // How much to grow on the x-Axis
    #[structopt(short = "w", long = "frequency-width", default_value = "0.1")]
    frequency_width: f64,
    // How much to grow on the y-Axis
    #[structopt(
        short = "h",
        long = "frequency-height",
        default_value = "0.23"
    )]
    frequency_height: f64,
}

pub struct RainbowWriter<T: Write + WriteColor> {
    writer: T,
    line: u64,
    character: u64,
    frequency_width: f64,
    frequency_height: f64,
    seed: u8,
    color_spec: ColorSpec,
}

impl<T: Write + WriteColor> RainbowWriter<T> {
    pub fn new(writer: T) -> RainbowWriter<T> {
        RainbowWriter {
            writer,
            line: 0,
            character: 0,
            seed: random(),
            color_spec: ColorSpec::new(),
            frequency_width: 0.1,
            frequency_height: 0.23,
        }
    }

    pub fn with_lol_opts(writer: T, lolopts: &LolOpts) -> RainbowWriter<T> {
        RainbowWriter {
            seed: lolopts.seed.unwrap_or_else(random),
            frequency_width: lolopts.frequency_width,
            frequency_height: lolopts.frequency_height,
            ..RainbowWriter::new(writer)
        }
    }

    fn rainbow(&self, width: u64, height: u64) -> Color {
        let position = width as f64 * self.frequency_width
            + height as f64 * self.frequency_height
            + f64::from(self.seed);

        let red = position.sin() * 127.0 + 128.0;
        let green = (position + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
        let blue = (position + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
        Color::Rgb(red as u8, green as u8, blue as u8)
    }
}

impl<T: Write + WriteColor> io::Write for RainbowWriter<T> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        for _byte in buffer {
            let byte = *_byte;
            if byte == b'\n' {
                self.line += 1;
                self.character = 0;
            }
            let color = self.rainbow(self.line, self.character);
            self.color_spec.set_fg(Some(color));
            self.writer.set_color(&self.color_spec)?;
            written += self.writer.write(&[byte])?;
        }
        Ok(written)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<T: Write + WriteColor> WriteColor for RainbowWriter<T> {
    fn supports_color(&self) -> bool {
        self.writer.supports_color()
    }
    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.writer.set_color(spec)
    }
    fn reset(&mut self) -> io::Result<()> {
        self.writer.reset()
    }
}
