mod rainbow;

#[cfg(windows)]
use ansi_term;
use rainbow::Rainbow;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "lolcat", about = "Terminal rainbows.")]
pub struct Cmdline {
    // If ANSI sequences should evaluated
    #[structopt(
        short = "A",
        long = "keep-ansi",
        help = "Don't filter ANSI sequences in input"
    )]
    keep_ansi: bool,
    #[structopt(name = "File", default_value = "-", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), io::Error> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    let opt = Cmdline::from_args();

    let files = if opt.files.is_empty() {
        vec![PathBuf::from("-")]
    } else {
        opt.files
    };
    let mut rainbow = Rainbow::default();
    rainbow.set_keep_ansi(opt.keep_ansi);

    for path in files {
        let stdin = io::stdin();
        let stdin = stdin.lock();
        let file: Box<dyn BufRead> = if path == PathBuf::from("-") {
            Box::new(stdin)
        } else {
            let f = File::open(path).unwrap();
            let b = BufReader::new(f);
            Box::new(b)
        };

        for line in file.lines().filter_map(|i| i.ok()) {
            let line = rainbow.colorize(&line);
            print!("{}", line)
        }
    }

    Ok(())
}
