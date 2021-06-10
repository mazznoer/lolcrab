use bstr::{io::BufReadExt, ByteSlice};
use std::io::{prelude::*, Write};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

pub struct Rainbow {
    current_row: usize,
    current_col: usize,
    shift_col: f64,
    shift_row: f64,
    position: f64,
    gradient: colorgrad::Gradient,
    invert: bool,
}

impl Rainbow {
    #[must_use]
    pub const fn new(
        gradient: colorgrad::Gradient,
        start: f64,
        shift_col: f64,
        shift_row: f64,
        invert: bool,
    ) -> Self {
        Self {
            gradient,
            shift_col,
            shift_row,
            position: start,
            current_row: 0,
            current_col: 0,
            invert,
        }
    }

    pub fn step_row(&mut self, n_row: usize) {
        self.current_row += n_row;
        self.position += n_row as f64 * self.shift_row;
    }

    pub fn step_col(&mut self, n_col: usize) {
        self.current_col += n_col;
        self.position += n_col as f64 * self.shift_col;
    }

    pub fn reset_row(&mut self) {
        self.position -= self.current_row as f64 * self.shift_row;
        self.current_row = 0;
    }

    pub fn reset_col(&mut self) {
        self.position -= self.current_col as f64 * self.shift_col;
        self.current_col = 0;
    }

    fn get_position(&mut self) -> f64 {
        if self.position < 0.0 || self.position > 1.0 {
            self.position -= self.position.floor();
        }

        self.position
    }

    pub fn get_color(&mut self) -> (u8, u8, u8) {
        let position = self.get_position();
        let (r, g, b, _) = self.gradient.at(position).rgba_u8();

        (r, g, b)
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
        if grapheme == "\n" {
            self.reset_col();
            self.step_row(1);
            if self.invert {
                out.write_all(b"\x1B[49m")?;
            }
            out.write_all(b"\n")?;
            return Ok(false);
        }

        if escaping {
            out.write_all(grapheme.as_bytes())?;
            escaping = grapheme.len() != 1 || {
                let c = grapheme.as_bytes()[0];
                !(b'a'..=b'z').contains(&c) && !(b'A'..=b'Z').contains(&c)
            };
        } else {
            let (r, g, b) = self.get_color();
            if self.invert {
                write!(out, "\x1B[38;2;0;0;0;48;2;{};{};{}m{}", r, g, b, grapheme)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_rb() -> Rainbow {
        Rainbow::new(colorgrad::rainbow(), 0.0, 0.1, 0.2, false)
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
    fn test_reset_row() {
        let mut rb_a = create_rb();
        let mut rb_b = create_rb();
        rb_a.step_row(20);
        rb_a.reset_row();
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
