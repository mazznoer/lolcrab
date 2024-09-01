#![warn(clippy::pedantic, clippy::nursery)]

use clap::{CommandFactory, Parser, ValueEnum};
use lolcrab::{Gradient, Lolcrab, Opt};
use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::PathBuf,
};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
    let opt = Opt::parse();
    let mut stdout = io::stdout().lock();
    let mut lol: Lolcrab = opt.clone().into();

    if opt.help {
        lol.colorize_str(
            &Opt::command().render_help().ansi().to_string(),
            &mut stdout,
        )?;
        return Ok(());
    }

    if opt.version {
        lol.colorize_str(&Opt::command().render_long_version(), &mut stdout)?;
        return Ok(());
    }

    if opt.presets {
        for g in Gradient::value_variants() {
            let name = format!("{g:?}").to_lowercase();
            let name = if name == "rdylgn" { "rd-yl-gn" } else { &name };
            writeln!(stdout, "\n{name}\n")?;
            lol.gradient = g.to_gradient();
            lol.colorize_str(SAMPLE_TEXT, &mut stdout)?;
        }
        return Ok(());
    }

    for path in opt.files {
        if path == PathBuf::from("-") {
            let mut stdin = io::stdin().lock();
            if opt.animate {
                lol.colorize_read_anim(&mut stdin, &mut stdout)?;
            } else {
                lol.colorize_read(&mut stdin, &mut stdout)?;
            }
        } else {
            let f = File::open(path).unwrap();
            let mut b = BufReader::new(f);
            if opt.animate {
                lol.colorize_read_anim(&mut b, &mut stdout)?;
            } else {
                lol.colorize_read(&mut b, &mut stdout)?;
            }
        }
    }

    Ok(())
}
