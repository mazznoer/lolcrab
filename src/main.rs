#![warn(clippy::pedantic, clippy::nursery)]

use clap::Parser;
use lolcrab::{Rainbow, RainbowCmd};
use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Parser)]
#[command(name = "lolcrab", version, about)]
pub struct Cmdline {
    #[arg(name = "File", default_value = "-", value_parser = clap::value_parser!(PathBuf))]
    files: Vec<PathBuf>,

    #[command(flatten)]
    rainbow: RainbowCmd,
}

fn main() -> Result<(), io::Error> {
    let opt = Cmdline::parse();

    let mut rainbow: Rainbow = opt.rainbow.into();

    for path in opt.files {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        if path == PathBuf::from("-") {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            rainbow.colorize_read(&mut stdin, &mut stdout)?;
        } else {
            let f = File::open(path).unwrap();
            let mut b = BufReader::new(f);
            rainbow.colorize_read(&mut b, &mut stdout)?;
        }
    }

    Ok(())
}
