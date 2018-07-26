#![feature(rust_2018_preview, use_extern_macros)]

mod rainbow;

#[cfg(windows)]
use ansi_term;
use crate::rainbow::RainbowWriter;
use std::fs::File;
use std::io::{self, BufReader};
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "lolcat", about = "Terminal rainbows.")]
struct Cmdline {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(flatten)]
    rainbow_writer: RainbowWriter,
}

fn main() -> Result<(), io::Error> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    let opt = Cmdline::from_args();

    let stdout = io::stdout();
    let out = stdout.lock();
    if opt.input.is_some() && Some("-".into()) != opt.input {
        let f = File::open(opt.input.unwrap())?;
        opt.rainbow_writer.rainbow_copy(BufReader::new(f), out)
    } else {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        opt.rainbow_writer.rainbow_copy(stdin_lock, out)
    }
}
