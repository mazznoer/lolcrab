use crate::Rainbow;
use clap::{ArgEnum, Clap};

#[derive(Debug, ArgEnum)]
pub enum RainbowStyle {
    Rainbow,
    Sinebow,
}

#[derive(Debug, Clap)]
pub struct RainbowCmd {
    ///  How many degrees to shift text color hue for every column
    #[clap(short = 'C', long, default_value = "1.6")]
    shift_col: f64,

    /// How many degrees to shift text color hue for every row
    #[clap(short = 'R', long, default_value = "3.2")]
    shift_row: f64,

    /// Don't randomize sign of col and row shift values
    #[clap(short = 'n', long)]
    shift_sign_no_random: bool,

    /// Sets initial hue of text color in degress [default: random]
    #[clap(short = 'H', long)]
    hue: Option<f64>,

    /// Rainbow mode
    #[clap(short, long, arg_enum, default_value = "rainbow")]
    style: RainbowStyle,

    /// Sets seed [default: random]
    #[clap(short = 'S', long)]
    seed: Option<u64>,
}

impl From<RainbowCmd> for Rainbow {
    fn from(cmd: RainbowCmd) -> Rainbow {
        if let Some(seed) = cmd.seed {
            fastrand::seed(seed);
        }

        let shift_col = if cmd.shift_sign_no_random || fastrand::bool() {
            cmd.shift_col
        } else {
            -cmd.shift_col
        } / 360.;

        let shift_row = if cmd.shift_sign_no_random || fastrand::bool() {
            cmd.shift_row
        } else {
            -cmd.shift_row
        } / 360.;

        let start = cmd.hue.map(|hue| hue / 360.).unwrap_or_else(fastrand::f64);

        let grad = match cmd.style {
            RainbowStyle::Rainbow => colorgrad::rainbow(),
            RainbowStyle::Sinebow => colorgrad::sinebow(),
        };

        Rainbow::new(grad, start, shift_col, shift_row)
    }
}
