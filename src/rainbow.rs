use palette::{encoding::pixel::Pixel, Hsl, Hue, RgbHue, Srgb};
use rand::prelude::*;
use std::io::{self, prelude::*, Bytes};
use structopt::StructOpt;
use unicode_reader::{CodePoints, Graphemes};
use unicode_width::UnicodeWidthStr;

#[derive(StructOpt)]
pub struct RainbowOpts {
    /// Set inital hue (in degrees) [default: random]
    #[structopt(short = "H", long = "hue")]
    hue: Option<f32>,
    #[structopt(
        short = "s",
        long = "saturation",
        default_value = "1.0",
        help = "Saturation of colors in rainbow"
    )]
    saturation: f32,
    #[structopt(
        short = "l",
        long = "lighness",
        default_value = "0.5",
        help = "Lighness of colors in rainbow"
    )]
    lightness: f32,
    #[structopt(
        short = "c",
        long = "shift-column",
        default_value = "1.6",
        help = "How much to shift color for every column"
    )]
    shift_column: f32,
    #[structopt(
        short = "r",
        long = "shift-row",
        default_value = "2.2",
        help = "How much to shift color for every row"
    )]
    shift_row: f32,
    #[structopt(
        short = "S",
        long = "disable-random-sign",
        help = "Disable random sign for column and row shift"
    )]
    random_sign: bool,
}

struct RainbowState {
    character_count: usize,
    shift_column: f32,
    shift_row: f32,
    color: Hsl,
}

pub struct RainbowWriter<R: BufRead, W: Write> {
    rainbow_state: RainbowState,
    writer: W,
    reader: Graphemes<CodePoints<Bytes<R>>>,
}

impl<R: BufRead, W: Write> RainbowWriter<R, W> {
    pub fn with_opts(reader: R, writer: W, opts: &RainbowOpts) -> RainbowWriter<R, W> {
        let mut rng = SmallRng::from_entropy();
        let sign_col: f32 = if opts.random_sign {
            rng.gen_range::<i8>(-1, 1) as f32
        } else {
            1.0
        };
        let sign_row: f32 = if opts.random_sign {
            rng.gen_range::<i8>(-1, 1) as f32
        } else {
            1.0
        };
        RainbowWriter {
            reader: Graphemes::from(reader),
            writer,
            rainbow_state: RainbowState {
                color: Hsl::new(
                    RgbHue::from_degrees(opts.hue.unwrap_or_else(|| rng.gen_range(0.0, 360.0))),
                    opts.saturation,
                    opts.lightness,
                ),
                shift_column: sign_col * opts.shift_column,
                shift_row: sign_row * opts.shift_row,
                character_count: 0,
            },
        }
    }

    pub fn rainbow_copy(mut self) -> Result<(), io::Error> {
        let mut print_color = true;
        for grapheme in self.reader.filter_map(|g| g.ok()) {
            if grapheme == "\x1B" {
                print_color = false;
            }

            if print_color {
                if grapheme == "\n" {
                    self.rainbow_state.bump_line();
                } else {
                    self.rainbow_state.bump_char(&grapheme);
                }
                let [r, g, b] = self.rainbow_state.next_color();
                write!(self.writer, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
            } else {
                if "a" <= &grapheme && "z" >= &grapheme || "A" <= &grapheme && "Z" >= &grapheme {
                    print_color = true;
                }
                write!(self.writer, "{}", grapheme)?;
            }
        }
        self.writer.write_all(b"\x1B[0m")?;
        self.writer.flush()
    }

    pub fn rainbow_copy_no_ansi(mut self) -> Result<(), io::Error> {
        for grapheme in self.reader.filter_map(|g| g.ok()) {
            if grapheme == "\n" {
                self.rainbow_state.bump_line();
            } else {
                self.rainbow_state.bump_char(&grapheme);
            }

            let [r, g, b] = self.rainbow_state.next_color();
            write!(self.writer, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
        }
        self.writer.write_all(b"\x1B[0m")
    }
}

impl RainbowState {
    #[inline]
    fn bump_line(&mut self) {
        let char_count = std::mem::replace(&mut self.character_count, 0);
        self.color = self.color.shift_hue(RgbHue::from_degrees(
            self.shift_row - char_count as f32 * self.shift_column,
        ));
    }

    #[inline]
    fn bump_char(&mut self, string: &str) {
        let width = UnicodeWidthStr::width(string);
        self.character_count += width;
        self.color = self
            .color
            .shift_hue(RgbHue::from_degrees(width as f32 * self.shift_column));
    }

    #[inline]
    fn next_color(&mut self) -> [u8; 3] {
        Srgb::from_linear(self.color.into())
            .into_format()
            .into_raw()
    }
}
