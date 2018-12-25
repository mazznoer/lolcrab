mod cat;
mod state;

use crate::cat::{rainbow_copy, rainbow_copy_no_ansi, RainbowOpts};
#[cfg(windows)]
use ansi_term;
use std::{
    alloc::System,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

#[global_allocator]
static GLOBAL: System = System;

#[derive(StructOpt)]
#[structopt(name = "lolcat", about = "Terminal rainbows.")]
struct Cmdline {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    // If ANSI sequences should evaluated
    #[structopt(
        short = "A",
        long = "skip-ansi",
        help = "Don't evalute ANSI sequences in input"
    )]
    dont_parse_ansi: bool,

    #[structopt(flatten)]
    lol_options: RainbowOpts,
}

fn main() -> Result<(), io::Error> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    let opt = Cmdline::from_args();

    let stdin = io::stdin();
    let input: Box<BufRead> = if opt.input.is_some() && Some("-".into()) != opt.input {
        let f = File::open(opt.input.unwrap())?;
        Box::new(BufReader::new(f))
    } else {
        Box::new(stdin.lock())
    };

    let stdout = io::stdout();
    let writer = stdout.lock();

    if opt.dont_parse_ansi {
        rainbow_copy_no_ansi(input, writer, &opt.lol_options)
    } else {
        rainbow_copy(input, writer, &opt.lol_options)
    }
}
