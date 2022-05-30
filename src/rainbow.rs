use bstr::{io::BufReadExt, ByteSlice};
use colorgrad::Color;
use noise::{NoiseFn, Seedable};
use std::io::{prelude::*, Write};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

pub struct Rainbow {
    current_row: usize,
    current_col: usize,
    gradient: colorgrad::Gradient,
    noise: noise::OpenSimplex,
    noise_scale: f64,
    invert: bool,
}

impl Rainbow {
    #[must_use]
    pub fn new(gradient: colorgrad::Gradient, noise_scale: f64, invert: bool) -> Self {
        Self {
            gradient,
            noise: noise::OpenSimplex::new().set_seed(fastrand::u32(..)),
            current_row: 0,
            current_col: 0,
            noise_scale,
            invert,
        }
    }

    pub fn step_row(&mut self, n_row: usize) {
        self.current_row += n_row;
    }

    pub fn step_col(&mut self, n_col: usize) {
        self.current_col += n_col;
    }

    pub fn reset_col(&mut self) {
        self.current_col = 0;
    }

    fn get_position(&mut self) -> f64 {
        self.noise.get([
            self.current_col as f64 * self.noise_scale,
            self.current_row as f64 * self.noise_scale * 2.0,
        ]) + 0.5
    }

    pub fn get_color(&mut self) -> Color {
        let position = self.get_position();
        self.gradient.at(position)
    }

    #[inline]
    fn handle_grapheme(
        &mut self,
        out: &mut impl Write,
        grapheme: &str,
        escaping: bool,
    ) -> std::io::Result<bool> {
        let mut escaping = escaping;
        if grapheme == "\x1B" {
            out.write_all(b"\x1B")?;
            return Ok(true);
        }

        if grapheme == "\n" || grapheme == "\r\n" {
            self.reset_col();
            self.step_row(1);
            if self.invert {
                out.write_all(b"\x1B[49m")?;
            }
            out.write_all(grapheme.as_bytes())?;
            return Ok(false);
        }

        if escaping {
            out.write_all(grapheme.as_bytes())?;
            escaping = grapheme.len() != 1 || {
                let c = grapheme.as_bytes()[0];
                !(b'a'..=b'z').contains(&c) && !(b'A'..=b'Z').contains(&c)
            };
        } else {
            let col = self.get_color();
            let (r, g, b, _) = col.rgba_u8();

            if self.invert {
                let lum = get_luminance(&col);

                let fg = if lum < 0.2 {
                    set_luminance(&col, lum + 0.25)
                } else {
                    set_luminance(&col, lum - 0.35)
                }
                .rgba_u8();

                write!(
                    out,
                    "\x1B[38;2;{};{};{};48;2;{};{};{}m{}",
                    fg.0, fg.1, fg.2, r, g, b, grapheme
                )?;
            } else {
                write!(out, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
            }
            self.step_col(
                grapheme
                    .chars()
                    .next()
                    .and_then(UnicodeWidthChar::width)
                    .unwrap_or(0),
            );
        }
        Ok(escaping)
    }

    /// # Errors
    ///
    /// Will return `Err` if `out` causes I/O erros
    pub fn colorize(&mut self, text: &[u8], out: &mut impl Write) -> std::io::Result<()> {
        let mut escaping = false;
        for grapheme in text.graphemes() {
            escaping = self.handle_grapheme(out, grapheme, escaping)?;
        }

        out.write_all(b"\x1B[39m")?;
        if self.invert {
            out.write_all(b"\x1B[49m")?;
        }
        out.flush()
    }

    /// # Errors
    ///
    /// Will return `Err` if `out` causes I/O erros
    pub fn colorize_str(&mut self, text: &str, out: &mut impl Write) -> std::io::Result<()> {
        let mut escaping = false;
        for grapheme in UnicodeSegmentation::graphemes(text, true) {
            escaping = self.handle_grapheme(out, grapheme, escaping)?;
        }

        out.write_all(b"\x1B[39m")?;
        if self.invert {
            out.write_all(b"\x1B[49m")?;
        }
        out.flush()
    }

    /// # Errors
    ///
    /// Will return `Err` if `input` or `out` cause I/O errors
    pub fn colorize_read(
        &mut self,
        input: &mut impl BufRead,
        out: &mut impl Write,
    ) -> std::io::Result<()> {
        input.for_byte_line_with_terminator(|line| {
            self.colorize(line, out)?;
            Ok(true)
        })
    }
}

fn get_luminance(col: &Color) -> f64 {
    let (r, g, b, _) = col.rgba();
    0.299 * r + 0.587 * g + 0.114 * b
}

fn set_luminance(col: &Color, lum: f64) -> Color {
    // https://github.com/gka/chroma.js/blob/master/src/ops/luminance.js

    if lum <= 0.0 {
        return Color::from_rgb(0.0, 0.0, 0.0);
    }

    if lum >= 1.0 {
        return Color::from_rgb(1.0, 1.0, 1.0);
    }

    let cur_lum = get_luminance(col);

    let (mut low, mut high) = if cur_lum > lum {
        (Color::from_rgb(0.0, 0.0, 0.0), col.clone())
    } else {
        (col.clone(), Color::from_rgb(1.0, 1.0, 1.0))
    };

    for i in 1..=30 {
        let mid = low.interpolate_rgb(&high, 0.5);
        let lm = get_luminance(&mid);

        if (lum - lm).abs() < f64::EPSILON || i == 30 {
            return mid;
        }

        if lm > lum {
            high = mid;
        } else {
            low = mid;
        }
    }

    col.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_rb() -> Rainbow {
        fastrand::seed(0);
        Rainbow::new(colorgrad::rainbow(), 0.03, false)
    }

    #[test]
    fn test_eq_str_u8() {
        let test = "foobar";

        let mut rb_a = create_rb();
        let mut out_a = Vec::new();
        rb_a.colorize(&test.as_bytes(), &mut out_a).unwrap();

        let mut rb_b = create_rb();
        let mut out_b = Vec::new();
        rb_b.colorize_str(&test, &mut out_b).unwrap();

        assert_eq!(out_a, out_b);
    }

    #[test]
    fn test_char_width() {
        let test = "f";
        let mut rb_a = create_rb();
        rb_a.colorize_str(&test, &mut Vec::new()).unwrap();

        assert_eq!(rb_a.current_col, 1);

        let test = "😃";
        let mut rb_b = create_rb();
        rb_b.colorize_str(&test, &mut Vec::new()).unwrap();
        assert_eq!(rb_b.current_col, 2);
    }

    #[test]
    fn test_step_row() {
        let test_string = "foobar\n";

        let mut rb_a = create_rb();
        rb_a.colorize(&test_string.as_bytes(), &mut Vec::new())
            .unwrap();
        let mut rb_b = create_rb();
        rb_b.step_row(1);
        assert_eq!(rb_a.get_color(), rb_b.get_color(),);
    }

    #[test]
    fn test_reset_col() {
        let mut rb_a = create_rb();
        let mut rb_b = create_rb();
        rb_a.step_col(20);
        rb_a.reset_col();
        assert_eq!(rb_a.get_color(), rb_b.get_color(),);
    }
}
