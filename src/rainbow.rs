use bstr::ByteSlice;
use scarlet::{colors::cielchcolor::CIELCHColor, prelude::*};
use std::io::Write;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
pub struct Rainbow {
    current_row: i32,
    current_col: i32,
    shift_col: f64,
    shift_row: f64,

    pub color: CIELCHColor,
}

impl Rainbow {
    pub fn new(start_color: &impl Color, shift_col: f64, shift_row: f64) -> Self {
        let color: CIELCHColor = start_color.convert();

        Self {
            color,
            shift_col,
            shift_row,
            current_col: 0,
            current_row: 0,
        }
    }

    pub fn step_row(&mut self, n_row: i32) {
        self.current_row += n_row;
        self.color
            .set_hue(self.color.hue() + (n_row as f64) * self.shift_row);
    }

    pub fn step_col(&mut self, n_col: i32) {
        self.current_col += n_col;
        self.color
            .set_hue(self.color.hue() + (n_col as f64) * self.shift_col);
    }

    #[allow(dead_code)]
    pub fn reset_row(&mut self) {
        self.step_row(-self.current_row)
    }

    pub fn reset_col(&mut self) {
        self.step_col(-self.current_col)
    }

    pub fn colorize(&mut self, text: &[u8], out: &mut dyn Write) -> std::io::Result<()> {
        let mut escaping = false;
        for grapheme in text.graphemes() {
            if grapheme == "\x1B" {
                escaping = true;
                continue;
            }
            if grapheme == "\n" {
                self.reset_col();
                self.step_row(1);
                writeln!(out).unwrap();
                continue;
            }

            if !escaping {
                self.step_col(UnicodeWidthStr::width(grapheme) as i32);
                let (r, g, b) = RGBColor::clamp(self.color)
                    .convert::<RGBColor>()
                    .int_rgb_tup();
                write!(out, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
            } else if "a" <= grapheme && "z" >= grapheme || "A" <= grapheme && "Z" >= grapheme {
                escaping = false;
            }
        }

        out.write_all(b"\x1B[0m")
    }

    #[allow(dead_code)]
    pub fn colorize_str(&mut self, text: &str, out: &mut dyn Write) -> std::io::Result<()> {
        let mut escaping = false;
        for grapheme in UnicodeSegmentation::graphemes(text, true) {
            if grapheme == "\x1B" {
                escaping = true;
                continue;
            }
            if grapheme == "\n" {
                self.reset_col();
                self.step_row(1);
                writeln!(out).unwrap();
                continue;
            }

            if !escaping {
                self.step_col(UnicodeWidthStr::width(grapheme) as i32);
                let (r, g, b) = RGBColor::clamp(self.color)
                    .convert::<RGBColor>()
                    .int_rgb_tup();
                write!(out, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
            } else if "a" <= grapheme && "z" >= grapheme || "A" <= grapheme && "Z" >= grapheme {
                escaping = false;
            }
        }

        out.write_all(b"\x1B[0m")
    }
}
