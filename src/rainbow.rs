use palette::{encoding::pixel::Pixel, white_point::D65, Hue, LabHue, Lch, Srgb};
use rand::prelude::*;
use std::fmt::Write;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct Rainbow {
    character_count: usize,
    shift_column: f32,
    shift_row: f32,
    color: Lch<D65, f32>,
    keep_ansi: bool,
}

impl Default for Rainbow {
    fn default() -> Self {
        let mut rng = SmallRng::from_entropy();
        let shift_column = if rng.gen() { 1.6 } else { -1.6 };
        let shift_row = if rng.gen() { 2.2 } else { -2.2 };
        let color = Lch::new(50.0, 128.0, LabHue::from_degrees(rng.gen_range(0.0, 360.0)));
        Self {
            color,
            shift_column,
            shift_row,
            keep_ansi: false,
            character_count: 0,
        }
    }
}

impl Rainbow {
    #[inline]
    fn current_color(&mut self) -> [u8; 3] {
        Srgb::from_linear(self.color.into())
            .into_format()
            .into_raw()
    }

    fn bump_char(&mut self, string: &str) -> [u8; 3] {
        let width = UnicodeWidthStr::width(string);
        self.character_count += width;
        self.color = self
            .color
            .shift_hue(LabHue::from_degrees(width as f32 * self.shift_column));
        self.current_color()
    }

    fn bump_line(&mut self) {
        let char_count = std::mem::replace(&mut self.character_count, 0);
        self.color = self.color.shift_hue(LabHue::from_degrees(
            self.shift_row - char_count as f32 * self.shift_column,
        ));
    }

    pub fn set_keep_ansi(&mut self, keep_ansi: bool) {
        self.keep_ansi = keep_ansi;
    }

    pub fn colorize(&mut self, text: &str) -> String {
        let mut escaping = false;
        let mut out = String::new();
        UnicodeSegmentation::graphemes(text, true).for_each(|grapheme| {
            if !self.keep_ansi && grapheme == "\x1B" {
                escaping = true;
                return;
            }
            if grapheme == "\n" {
                self.bump_line();
                writeln!(out).unwrap();
                return;
            }

            if self.keep_ansi || !escaping {
                let [r, g, b] = self.bump_char(&grapheme);
                write!(out, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme).unwrap();
            } else if "a" <= grapheme && "z" >= grapheme || "A" <= grapheme && "Z" >= grapheme {
                escaping = false;
            }
        });
        writeln!(out).unwrap();
        self.bump_line();
        write!(out, "\x1B[0m").unwrap();

        out
    }
}
