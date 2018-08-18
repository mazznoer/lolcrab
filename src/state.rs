use crate::cat::RainbowOpts;
use palette::{encoding::pixel::Pixel, Hsl, Hue, RgbHue, Srgb};
use rand::prelude::*;
use unicode_width::UnicodeWidthStr;

pub struct State {
    character_count: usize,
    shift_column: f32,
    shift_row: f32,
    color: Hsl,
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
            color: Hsl::new(
                RgbHue::from_degrees(opts.hue.unwrap_or_else(|| rng.gen_range(0.0, 360.0))),
                opts.saturation,
                opts.lightness,
            ),
            shift_column: sign_col * opts.shift_column,
            shift_row: sign_row * opts.shift_row,
            character_count: 0,
        }
    }

    pub fn feed(&mut self, string: &str) -> [u8; 3] {
        match string {
            "\n" | "\r\n" => self.bump_line(),
            _ => self.bump_char(string),
        };
        self.current_color()
    }

    #[inline]
    pub fn current_color(&mut self) -> [u8; 3] {
        Srgb::from_linear(self.color.into())
            .into_format()
            .into_raw()
    }

    #[inline]
    fn bump_line(&mut self) {
        let char_count = std::mem::replace(&mut self.character_count, 0);
        self.color = self.color.shift_hue(RgbHue::from_degrees(
            self.shift_row - char_count as f32 * self.shift_column,
        ));
    }

    #[inline]
    fn bump_char(&mut self, string: &str) {
        let width = UnicodeWidthStr::width(string);
        self.character_count += width;
        self.color = self
            .color
            .shift_hue(RgbHue::from_degrees(width as f32 * self.shift_column));
    }
}
