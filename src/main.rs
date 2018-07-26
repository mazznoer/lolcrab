#![feature(rust_2018_preview, use_extern_macros)]

mod rainbow;

use crate::rainbow::{RainbowOpts, RainbowWriter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;
use termcolor::{ColorChoice, StandardStream};

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
    let opt = Cmdline::from_args();

    let stdin = io::stdin();
    let input: Box<BufRead> = if opt.input.is_some() && Some("-".into()) != opt.input {
        let f = File::open(opt.input.unwrap())?;
        Box::new(BufReader::new(f))
    } else {
        Box::new(stdin.lock())
    };

    let outstream = StandardStream::stdout(ColorChoice::Always);

    let rainbow = RainbowWriter::with_opts(outstream.lock(), input, &opt.lol_options);
    rainbow.rainbow_copy()
}
