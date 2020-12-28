use crate::{color::*, Rainbow};
use clap::Clap;
use rand::prelude::*;
use std::f64::consts::PI;

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
    #[clap(short, long)]
    hue: Option<f64>,

    /// Sets text color luminance
    #[clap(short, long, default_value = "0.85")]
    luminance: f64,

    /// Sets text color chroma
    #[clap(short, long, default_value = "0.5")]
    chroma: f64,
}

impl Into<Rainbow> for RainbowCmd {
    fn into(self) -> Rainbow {
        let mut rng = SmallRng::from_entropy();
        let hue = self
            .hue
            .map(f64::to_radians)
            .unwrap_or_else(|| rng.gen_range(-PI..PI));

        let color = LCh {
            L: self.luminance,
            C: self.chroma,
            h: hue,
        };

        let shift_col = if self.shift_sign_no_random || rng.gen() {
            self.shift_col
        } else {
            -self.shift_col
        };

        let shift_row = if self.shift_sign_no_random || rng.gen() {
            self.shift_col
        } else {
            -self.shift_col
        };

        Rainbow::new(
            &color,
            (shift_col * (u32::MAX as f64 / 360.)) as i32,
            (shift_row * (u32::MAX as f64 / 360.)) as i32,
        )
    }
}
