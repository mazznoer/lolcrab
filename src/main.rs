#![feature(rust_2018_preview, use_extern_macros)]

mod rainbow;

use crate::rainbow::{RainbowOpts, RainbowWriter};
use std::fs::File;
use std::io;
use std::io::BufReader;
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

    let outstream = StandardStream::stdout(ColorChoice::Always);
    let mut out = RainbowWriter::with_opts(outstream.lock(), &opt.lol_options);

    if opt.input.is_some() && Some("-".into()) != opt.input {
        let f = File::open(opt.input.unwrap())?;
        let mut file = BufReader::new(&f);
        io::copy(&mut file, &mut out)?;
    } else {
        let stdin = io::stdin();
        let mut input = stdin.lock();
        io::copy(&mut input, &mut out)?;
    }

    Ok(())
}
