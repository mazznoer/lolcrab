use rand::random;
use std::f64::consts::PI;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;
use termcolor::{Color, ColorSpec, WriteColor};
use unicode_reader::Graphemes;
use vte;

#[derive(StructOpt)]
pub struct RainbowOpts {
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

struct Performer<T: Write + WriteColor> {
    writer: T,
    line: u64,
    character: u64,
    frequency_width: f64,
    frequency_height: f64,
    seed: u8,
    color_spec: ColorSpec,
}

pub struct RainbowWriter<R: BufRead, W: Write + WriteColor> {
    performer: Performer<W>,
    reader: R,
    parser: vte::Parser,
}

impl<W: Write + WriteColor, R: BufRead> RainbowWriter<R, W> {
    pub fn with_opts(writer: W, reader: R, opts: &RainbowOpts) -> RainbowWriter<R, W> {
        RainbowWriter {
            reader,
            performer: Performer {
                writer,
                seed: opts.seed.unwrap_or_else(random),
                frequency_width: opts.frequency_width,
                frequency_height: opts.frequency_height,
                line: 0,
                character: 0,
                color_spec: ColorSpec::new(),
            },
            parser: vte::Parser::new(),
        }
    }

    pub fn rainbow_copy(mut self) -> Result<(), io::Error> {
        let graphemes = Graphemes::from(self.reader);
        for g in graphemes {
            let grapheme = g?;
            if grapheme.len() == 1 {
                self.parser
                    .advance(&mut self.performer, *grapheme.as_bytes().get(0).unwrap());
            } else {
                self.performer.next_color();
                self.performer.character += 1;
            }

            self.performer.writer.write_all(grapheme.as_bytes())?;
        }
        self.performer.writer.reset()
    }
}

impl<T: Write + WriteColor> Performer<T> {
    fn next_color(&mut self) {
        let position = self.character as f64 * self.frequency_width
            + self.line as f64 * self.frequency_height
            + f64::from(self.seed);

        let red = position.sin() * 127.0 + 128.0;
        let green = (position + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
        let blue = (position + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;

        let color = Color::Rgb(red as u8, green as u8, blue as u8);

        self.color_spec.set_fg(Some(color));
        self.writer.set_color(&self.color_spec).unwrap();
    }

    fn add_line(&mut self, x: u64) {
        self.line += x;
    }

    fn sub_line(&mut self, x: u64) {
        if self.line > x {
            self.line -= x;
            return;
        }
        self.line = 0;
    }

    fn add_char(&mut self, x: u64) {
        self.character += x;
    }

    fn sub_char(&mut self, x: u64) {
        if self.character > x {
            self.character -= x;
            return;
        }
        self.character = 0;
    }
}

impl<T: Write + WriteColor> vte::Perform for Performer<T> {
    fn print(&mut self, _c: char) {
        self.next_color();
        self.character += 1;
    }

    fn execute(&mut self, byte: u8) {
        const BS: u8 = 0x08;
        const HT: u8 = 0x09;
        const LF: u8 = 0x0A;
        const VT: u8 = 0x0B;
        const FF: u8 = 0x0C;
        const CR: u8 = 0x0D;
        const NEL: u8 = 0x85;
        const HTS: u8 = 0x88;
        match byte {
            HT | HTS => self.add_char(8),
            BS => self.sub_char(1),
            CR => self.character = 0,
            LF | VT | FF | NEL => {
                self.add_line(1);
                self.character = 0;
            }
            _ => (),
        }
    }

    fn csi_dispatch(&mut self, args: &[i64], _intermediates: &[u8], _ignore: bool, action: char) {
        let arg = args
            .get(0)
            .and_then(|v| if *v == 0 { None } else { Some(*v) })
            .unwrap_or(1) as u64;
        match action {
            '@' | 'b' => self.add_char(arg),
            'A' => self.sub_line(arg),
            'B' | 'e' | 'C' | 'a' => self.add_line(arg),
            'D' => self.sub_char(arg),
            'E' => {
                self.add_line(arg);
                self.character = 0;
            }
            'F' => {
                self.sub_line(arg);
                self.character = 0;
            }
            'G' | '`' => self.character = arg - 1,
            'H' | 'f' => {
                self.line = arg;
                self.character = args
                    .get(1)
                    .and_then(|v| if *v == 0 { None } else { Some(*v) })
                    .unwrap_or(1) as u64;
            }
            'I' => self.add_char(8 * arg),
            'L' => self.add_line(arg),
            'M' => self.character = 0,
            'Z' => self.sub_char(8 * arg),
            'd' => self.line = arg - 1,
            _ => (),
        }
    }

    fn esc_dispatch(&mut self, _params: &[i64], _intermediates: &[u8], _ignore: bool, byte: u8) {
        match byte {
            b'D' | b'E' => {
                self.add_line(1);
                self.character = 0;
            }
            b'M' => self.sub_line(1),
            _ => (),
        }
    }

    // Ignored
    fn hook(&mut self, _params: &[i64], _intermediates: &[u8], _ignore: bool) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]]) {}
}
