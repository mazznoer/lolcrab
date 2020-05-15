use crate::Rainbow;
use rand::prelude::*;
use scarlet::{colors::CIELCHColor, prelude::*};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RainbowCmd {
    /// How much the hue of the color gets shifted every column
    #[structopt(short = "C", long, default_value = "1.6")]
    shift_col: f64,

    /// How much the hue of the color gets shifted every row
    #[structopt(short = "R", long, default_value = "3.2")]
    shift_row: f64,

    /// Don't randomize sign of col and row shift value
    #[structopt(short = "n", long)]
    shift_sign_no_random: bool,

    /// Sets initial hue as defined by CIE L*C*h Color Scale [default: random]
    #[structopt(short, long)]
    hue: Option<f64>,

    /// Sets initial luminance as defined by CIE L*C*h Color Scale
    #[structopt(short, long, default_value = "50")]
    luminance: f64,

    /// Sets initial chroma as defined by CIE L*C*h Color Scale
    #[structopt(short, long, default_value = "128")]
    chroma: f64,
}

impl Into<Rainbow> for RainbowCmd {
    fn into(self) -> Rainbow {
        let mut rng = SmallRng::from_entropy();
        let hue = self.hue.unwrap_or_else(|| rng.gen_range(0.0, 360.0));

        let color = CIELCHColor {
            l: self.luminance,
            c: self.chroma,
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

        Rainbow::new(color.convert(), shift_col, shift_row)
    }
}
