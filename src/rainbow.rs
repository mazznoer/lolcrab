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
                let lum = color_luminance(&col);

                let fg = if lum < 0.5 {
                    blend_color(
                        &Color::from_rgba(1.0, 1.0, 1.0, remap(lum, 0.0, 0.5, 0.35, 0.85)),
                        &col,
                    )
                } else {
                    blend_color(
                        &Color::from_rgba(0.0, 0.0, 0.0, remap(lum, 0.5, 1.0, 0.40, 0.35)),
                        &col,
                    )
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

// Reference http://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
fn color_luminance(col: &Color) -> f64 {
    fn lum(t: f64) -> f64 {
        if t <= 0.03928 {
            t / 12.92
        } else {
            ((t + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * lum(col.r) + 0.7152 * lum(col.g) + 0.0722 * lum(col.b)
}

fn blend_color(fg: &Color, bg: &Color) -> Color {
    Color::from_rgb(
        ((1.0 - fg.a) * bg.r) + (fg.a * fg.r),
        ((1.0 - fg.a) * bg.g) + (fg.a * fg.g),
        ((1.0 - fg.a) * bg.b) + (fg.a * fg.b),
    )
}

// Map t from range [a, b] to range [c, d]
fn remap(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    (t - a) * ((d - c) / (b - a)) + c
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
