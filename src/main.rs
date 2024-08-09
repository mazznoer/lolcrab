#![warn(clippy::pedantic, clippy::nursery)]

use clap::{Parser, ValueEnum};
use lolcrab::{Gradient, Opt, Rainbow};
use std::{
    fs::File,
    io::{self, BufReader, Write},
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
    rainbow: Opt,
}

const SAMPLE_TEXT: &str = "\
oooo............oooo...github.com/mazznoer/lolcrab...o8.
`888............`888...............................'888.
.888....ooooo....888....ooooo...oooo.d8b...oooo.....888oooo.
.888..d88'.`88b..888..d88'.`'Y8.`888''8P.`P..)88b...d88'.`88b.
.888..888...888..888..888........888.......oP'888...888...888.
.888..888...888..888..888....o8..888.....d8(..888...888...888.
o888o.`Y8bod8P'.o888o.`Y8bod8P'.d888b....`Y888''8o..`Y8bod8P.
";

fn main() -> Result<(), io::Error> {
    let opt = Cmdline::parse();

    if opt.rainbow.presets {
        let presets = [
            "cividis",
            "cool",
            "cubehelix",
            "fruits",
            "inferno",
            "magma",
            "plasma",
            "rainbow",
            "rd-yl-gn",
            "sinebow",
            "spectral",
            "turbo",
            "viridis",
            "warm",
        ];
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        for s in &presets {
            let mut opt = opt.rainbow.clone();
            opt.gradient = Gradient::from_str(s, true).unwrap();
            opt.custom = None;
            opt.random_colors = None;
            let mut rainbow: Rainbow = opt.into();
            writeln!(stdout, "\n{s}\n")?;
            rainbow.colorize_str(SAMPLE_TEXT, &mut stdout)?;
        }
        return Ok(());
    }

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
