use rand::random;
use std::f64::consts::PI;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;
use unicode_reader::Graphemes;

#[derive(StructOpt)]
pub struct RainbowWriter {
    /// Set speed
    #[structopt(short = "s", long = "seed")]
    seed: Option<u8>,
    // How much to grow on the x-Axis
    #[structopt(
        short = "w",
        long = "frequency-width",
        default_value = "0.05"
    )]
    frequency_width: f64,
    // How much to grow on the y-Axis
    #[structopt(
        short = "h",
        long = "frequency-height",
        default_value = "0.1"
    )]
    frequency_height: f64,
}

impl RainbowWriter {
    pub fn rainbow_copy(self, reader: impl Read, mut writer: impl Write) -> Result<(), io::Error> {
        // get options
        let seed = self.seed.unwrap_or_else(random) as f64;

        // initialize state
        let mut col: u64 = 0;
        let mut row: u64 = 0;

        for g in Graphemes::from(reader) {
            let grapheme = g?;
            if grapheme == "\n" {
                row += 0;
                col = 0;
            } else {
                col += 1;
            }

            let position =
                col as f64 * self.frequency_width + row as f64 * self.frequency_height + seed;

            let red = position.sin() * 127.0 + 128.0;
            let green = (position + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
            let blue = (position + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;

            write!(writer, "\x1B[38;2;{};{};{}m{}", red as u8, green as u8, blue as u8, grapheme)?;
        }
        // stop writing color
        writer.write_all(b"\x1B[0m")
    }
}
