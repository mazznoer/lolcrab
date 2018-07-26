#![feature(rust_2018_preview, use_extern_macros)]

mod rainbow;

#[cfg(windows)]
use ansi_term;
use crate::rainbow::{RainbowOpts, RainbowWriter};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "lolcat", about = "Terminal rainbows.")]
struct Cmdline {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

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

    let rainbow = RainbowWriter::with_opts(input, writer, &opt.lol_options);
    rainbow.rainbow_copy()
}
