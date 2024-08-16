#![warn(clippy::pedantic, clippy::nursery)]

#[cfg(feature = "cli")]
use clap::{CommandFactory, Parser, ValueEnum};
#[cfg(feature = "cli")]
use lolcrab::{Gradient, Lolcrab, Opt};
#[cfg(feature = "cli")]
use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::PathBuf,
};

#[cfg(all(feature = "cli", feature = "mimalloc"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "cli")]
const SAMPLE_TEXT: &str = "\
oooo............oooo...github.com/mazznoer/lolcrab...o8.
`888............`888...............................'888.
.888....ooooo....888....ooooo...oooo.d8b...oooo.....888oooo.
.888..d88'.`88b..888..d88'.`'Y8.`888''8P.`P..)88b...d88'.`88b.
.888..888...888..888..888........888.......oP'888...888...888.
.888..888...888..888..888....o8..888.....d8(..888...888...888.
o888o.`Y8bod8P'.o888o.`Y8bod8P'.d888b....`Y888''8o..`Y8bod8P.
";

#[cfg(feature = "cli")]
fn main() -> Result<(), io::Error> {
    let opt = Opt::parse();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if opt.help || opt.version {
        let mut lol = Lolcrab::new(None, None);
        if opt.help {
            lol.colorize_str(
                &Opt::command().render_help().ansi().to_string(),
                &mut stdout,
            )?;
        } else {
            lol.colorize_str(&Opt::command().render_long_version(), &mut stdout)?;
        }
        return Ok(());
    }

    if opt.presets {
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
        for s in &presets {
            let mut opt = opt.clone();
            opt.gradient = Gradient::from_str(s, true).unwrap();
            opt.custom = None;
            opt.random_colors = None;
            let mut lol: Lolcrab = opt.into();
            writeln!(stdout, "\n{s}\n")?;
            lol.colorize_str(SAMPLE_TEXT, &mut stdout)?;
        }
        return Ok(());
    }

    let animate = opt.animate;
    let mut lol: Lolcrab = opt.clone().into();

    for path in opt.files {
        if path == PathBuf::from("-") {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            if animate {
                lol.colorize_read_anim(&mut stdin, &mut stdout)?;
            } else {
                lol.colorize_read(&mut stdin, &mut stdout)?;
            }
        } else {
            let f = File::open(path).unwrap();
            let mut b = BufReader::new(f);
            if animate {
                lol.colorize_read_anim(&mut b, &mut stdout)?;
            } else {
                lol.colorize_read(&mut b, &mut stdout)?;
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {}
