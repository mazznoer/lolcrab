#![warn(clippy::pedantic, clippy::nursery)]

use std::ffi::OsString;
use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::path::PathBuf;

use clap::{CommandFactory, Parser, ValueEnum};
use lolcrab::{Gradient, Lolcrab, Opt};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn config_file() -> Option<PathBuf> {
    std::env::var("LOLCRAB_CONFIG_PATH")
        .ok()
        .map(PathBuf::from)
        .filter(|config_path| config_path.is_file())
        .or_else(|| Some(dirs::config_dir()?.join("lolcrab").join("config")))
}

fn read_config_file() -> Vec<OsString> {
    let mut args = Vec::new();
    let Some(path) = config_file() else {
        return args;
    };
    if !path.exists() {
        return args;
    }
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(line_args) = shlex::split(line) {
            args.extend(line_args.into_iter().map(OsString::from));
        } else {
            eprintln!("Failed to parse config line '{line}'");
        }
    }

    args
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
    let mut args_cfg = read_config_file();
    let mut args_cli = std::env::args_os();
    args_cfg.insert(0, args_cli.next().unwrap());
    args_cfg.extend(args_cli);

    let opt = Opt::parse_from(args_cfg);
    let mut stdout = io::stdout().lock();
    let is_terminal = stdout.is_terminal();
    let mut stdout = io::BufWriter::new(&mut stdout);

    let mut lol: Lolcrab = opt.clone().into();

    if opt.help {
        if opt.animate {
            lol.colorize_read_anim(
                &mut BufReader::new(Opt::command().render_help().to_string().as_bytes()),
                &mut stdout,
            )?;
        } else {
            lol.colorize_str(
                &Opt::command().render_help().ansi().to_string(),
                &mut stdout,
            )?;
        }
        stdout.flush()?;
        return Ok(());
    }

    if opt.version {
        lol.colorize_str(&Opt::command().render_long_version(), &mut stdout)?;
        stdout.flush()?;
        return Ok(());
    }

    if opt.config_file {
        let Some(cfg_path) = config_file() else {
            return Ok(());
        };
        let cfg_path = format!("{}\n", cfg_path.display());
        if is_terminal {
            lol.colorize_str(&cfg_path, &mut stdout)?;
        } else {
            write!(stdout, "{cfg_path}")?;
        }
        stdout.flush()?;
        return Ok(());
    }

    if opt.presets {
        for g in Gradient::value_variants() {
            let name = format!("{g:?}").to_lowercase();
            let name = if name == "rdylgn" { "rd-yl-gn" } else { &name };
            if is_terminal {
                writeln!(stdout, "\n{name}\n")?;
                lol.gradient = g.to_gradient();
                lol.randomize_position();
                lol.colorize_str(SAMPLE_TEXT, &mut stdout)?;
            } else {
                writeln!(stdout, "{name}")?;
            }
        }
        stdout.flush()?;
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

    stdout.flush()?;
    Ok(())
}
