mod codec;
mod state;

use crate::{codec::LolCodec, state::RainbowOpts};
#[cfg(windows)]
use ansi_term;
use futures::{self, future::Future, stream::Stream};
use std::{io, path::PathBuf};
use structopt::StructOpt;
use tokio::{self, codec::LinesCodec};

#[derive(StructOpt)]
#[structopt(name = "lolcat", about = "Terminal rainbows.")]
pub struct Cmdline {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    // If ANSI sequences should evaluated
    #[structopt(
        short = "A",
        long = "skip-ansi",
        help = "Don't evalute ANSI sequences in input"
    )]
    skip_ansi: bool,

    #[structopt(flatten)]
    lol_options: RainbowOpts,
}

fn main() -> Result<(), io::Error> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    let opt = Cmdline::from_args();

    let stdout = tokio::io::stdout();
    let writer = tokio::codec::FramedWrite::new(stdout, LolCodec::new(&opt));

    if opt.input.is_some() && Some("-".into()) != opt.input {
        let task = tokio::fs::File::open(opt.input.unwrap())
            .and_then(|f| tokio::codec::FramedRead::new(f, LinesCodec::new()).forward(writer))
            .then(|_| Ok(()));
        tokio::run(task)
    } else {
        let stdin = tokio::io::stdin();
        let reader = tokio::codec::FramedRead::new(stdin, LinesCodec::new());
        let task = reader.forward(writer);
        tokio::run(task.then(|_| Ok(())))
    };

    Ok(())
}
