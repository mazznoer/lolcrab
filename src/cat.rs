use crate::state::State;
use std::io::{self, prelude::*};
use structopt::StructOpt;
use unicode_reader::Graphemes;

#[derive(StructOpt)]
pub struct RainbowOpts {
    /// Set inital hue (in degrees) [default: random]
    #[structopt(short = "H", long = "hue")]
    pub hue: Option<f32>,
    #[structopt(
        short = "s",
        long = "saturation",
        default_value = "1.0",
        help = "Saturation of colors in rainbow"
    )]
    pub saturation: f32,
    #[structopt(
        short = "l",
        long = "lighness",
        default_value = "0.5",
        help = "Lighness of colors in rainbow"
    )]
    pub lightness: f32,
    #[structopt(
        short = "c",
        long = "shift-column",
        default_value = "1.6",
        help = "How much to shift color for every column"
    )]
    pub shift_column: f32,
    #[structopt(
        short = "r",
        long = "shift-row",
        default_value = "2.2",
        help = "How much to shift color for every row"
    )]
    pub shift_row: f32,
    #[structopt(
        short = "S",
        long = "disable-random-sign",
        help = "Disable random sign for column and row shift"
    )]
    pub disable_random_sign: bool,
}

fn new_graphme_iterator(reader: impl Read) -> impl Iterator<Item = String> {
    Graphemes::from(reader).filter_map(|g| g.ok())
}

pub fn rainbow_copy(
    reader: impl Read,
    mut writer: impl Write,
    opts: &RainbowOpts,
) -> Result<(), io::Error> {
    let mut print_color = true;
    let mut rainbow_state = State::from_opts(&opts);
    for grapheme in new_graphme_iterator(reader) {
        if grapheme == "\x1B" {
            print_color = false;
        }

        if print_color {
            let [r, g, b] = rainbow_state.feed(&grapheme);
            write!(writer, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
        } else {
            if "a" <= &grapheme && "z" >= &grapheme || "A" <= &grapheme && "Z" >= &grapheme {
                print_color = true;
            }
            write!(writer, "{}", grapheme)?;
        }
    }
    writer.write_all(b"\x1B[0m")
}

pub fn rainbow_copy_no_ansi(
    reader: impl Read,
    mut writer: impl Write,
    opts: &RainbowOpts,
) -> Result<(), io::Error> {
    let mut rainbow_state = State::from_opts(&opts);
    for grapheme in new_graphme_iterator(reader) {
        let [r, g, b] = rainbow_state.feed(&grapheme);
        write!(writer, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
    }
    writer.write_all(b"\x1B[0m")
}
