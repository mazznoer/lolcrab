use crate::state::State;
use std::io::{self, prelude::*};
use structopt::StructOpt;
use unicode_segmentation::UnicodeSegmentation;

#[derive(StructOpt)]
pub struct RainbowOpts {
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

pub fn rainbow_copy(
    reader: impl BufRead,
    mut writer: impl Write,
    opts: &RainbowOpts,
) -> Result<(), io::Error> {
    let mut print_color = true;
    let mut rainbow_state = State::from_opts(&opts);
    reader.lines().filter_map(|s| s.ok()).for_each(|line| {
        UnicodeSegmentation::graphemes(&*line, true).for_each(|grapheme| {
            if grapheme == "\x1B" {
                print_color = false;
            }

            if print_color {
                let [r, g, b] = rainbow_state.feed(&grapheme);
                write!(writer, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme).unwrap();;
            } else {
                if "a" <= &grapheme && "z" >= &grapheme || "A" <= &grapheme && "Z" >= &grapheme {
                    print_color = true;
                }
                write!(writer, "{}", grapheme).unwrap();
            }
        });
        print_color = true;
        writeln!(writer).unwrap();
    });
    writer.write_all(b"\x1B[0m")
}

pub fn rainbow_copy_no_ansi(
    reader: impl BufRead,
    mut writer: impl Write,
    opts: &RainbowOpts,
) -> Result<(), io::Error> {
    let mut rainbow_state = State::from_opts(&opts);
    reader.lines().filter_map(|s| s.ok()).for_each(|line| {
        UnicodeSegmentation::graphemes(&*line, true).for_each(|grapheme| {
            let [r, g, b] = rainbow_state.feed(&grapheme);
            write!(writer, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme).unwrap();
        });
        writeln!(writer).unwrap();
    });
    writer.write_all(b"\x1B[0m")
}
