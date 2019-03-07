use palette::{encoding::pixel::Pixel, white_point::D65, Hue, LabHue, Lch, Srgb};
use rand::prelude::*;
use structopt::StructOpt;
use unicode_width::UnicodeWidthStr;

#[derive(StructOpt)]
pub struct RainbowOpts {
    #[structopt(
        short = "c",
        long = "shift-column",
        default_value = "1.6",
        help = "How much to shift color for every column"
    )]
    pub shift_column: f32,
    #[structopt(
        short = "r",
        long = "shift-row",
        default_value = "2.2",
        help = "How much to shift color for every row"
    )]
    pub shift_row: f32,
    #[structopt(
        short = "S",
        long = "disable-random-sign",
        help = "Disable random sign for column and row shift"
    )]
    pub disable_random_sign: bool,
}

pub struct State {
    character_count: usize,
    shift_column: f32,
    shift_row: f32,
    color: Lch<D65, f32>,
}

impl State {
    pub fn from_opts(opts: &RainbowOpts) -> Self {
        let mut rng = SmallRng::from_entropy();
        let sign_col = if opts.disable_random_sign || rng.gen() {
            1.0
        } else {
            -1.0
        };
        let sign_row = if !opts.disable_random_sign || rng.gen() {
            1.0
        } else {
            -1.0
        };
        State {
            color: Lch::new(50.0, 128.0, LabHue::from_degrees(rng.gen_range(0.0, 360.0))),
            shift_column: sign_col * opts.shift_column,
            shift_row: sign_row * opts.shift_row,
            character_count: 0,
        }
    }

    pub fn feed(&mut self, string: &str) -> [u8; 3] {
        let width = UnicodeWidthStr::width(string);
        self.character_count += width;
        self.color = self
            .color
            .shift_hue(LabHue::from_degrees(width as f32 * self.shift_column));
        self.current_color()
    }

    #[inline]
    pub fn current_color(&mut self) -> [u8; 3] {
        Srgb::from_linear(self.color.into())
            .into_format()
            .into_raw()
    }

    pub fn bump_line(&mut self) {
        let char_count = std::mem::replace(&mut self.character_count, 0);
        self.color = self.color.shift_hue(LabHue::from_degrees(
            self.shift_row - char_count as f32 * self.shift_column,
        ));
    }
}
