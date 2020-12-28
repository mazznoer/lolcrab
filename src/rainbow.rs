use crate::color::*;
use bstr::{io::BufReadExt, ByteSlice};
use lru::LruCache;
use std::io::{prelude::*, Write};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

pub struct Rainbow {
    current_row: usize,
    current_col: usize,
    shift_col: i32,
    shift_row: i32,
    base_hue: u32,
    cache: LruCache<u32, RGB>,
    color: Lab,
}

impl Rainbow {
    pub fn new(color: impl Into<Lab>, shift_col: i32, shift_row: i32) -> Self {
        let color = color.into();
        let base_hue = color.hue_u32();

        Self {
            color,
            base_hue,
            shift_col,
            shift_row,
            current_col: 0,
            current_row: 0,
            cache: LruCache::new(512),
        }
    }

    pub fn step_row(&mut self, n_row: usize) {
        self.current_row += n_row;
    }

    pub fn step_col(&mut self, n_col: usize) {
        self.current_col += n_col;
    }

    pub fn reset_row(&mut self) {
        self.current_row = 0;
    }

    pub fn reset_col(&mut self) {
        self.current_col = 0;
    }

    pub fn get_color(&mut self) -> RGB {
        let mut hue = self.base_hue;

        hue = if self.shift_row >= 0 {
            hue.overflowing_add(
                (self.current_row as u32)
                    .overflowing_mul(self.shift_row as u32)
                    .0,
            )
        } else {
            hue.overflowing_sub(
                (self.current_row as u32)
                    .overflowing_mul(-self.shift_row as u32)
                    .0,
            )
        }
        .0;

        hue = if self.shift_col >= 0 {
            hue.overflowing_add(
                (self.current_col as u32)
                    .overflowing_mul(self.shift_col as u32)
                    .0,
            )
        } else {
            hue.overflowing_sub(
                (self.current_col as u32)
                    .overflowing_mul((-self.shift_col) as u32)
                    .0,
            )
        }
        .0;

        if let Some(out) = self.cache.get(&hue) {
            return out.clone();
        }

        self.color.set_hue_u32(hue);
        let out: RGB = (&self.color).into();

        self.cache.put(hue, out.clone());

        out
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
            out.write_all(b"\n")?;
            return Ok(false);
        }

        if !escaping {
            let color = self.get_color();
            write!(
                out,
                "\x1B[38;2;{};{};{}m{}",
                color.r, color.g, color.b, grapheme
            )?;
            self.step_col(grapheme.chars().next().and_then(|c| c.width()).unwrap_or(0));
        } else {
            // write!(out, "{}", grapheme)?;
            out.write_all(grapheme.as_bytes())?;
            escaping = grapheme.len() != 1 || {
                let c = grapheme.as_bytes()[0];
                !(b'a'..=b'z').contains(&c) && !(b'A'..=b'Z').contains(&c)
            };
        }
        Ok(escaping)
    }

    pub fn colorize(&mut self, text: &[u8], out: &mut impl Write) -> std::io::Result<()> {
        let mut escaping = false;
        for grapheme in text.graphemes() {
            escaping = self.handle_grapheme(out, grapheme, escaping)?;
        }

        out.write_all(b"\x1B[39m")?;
        out.flush()
    }

    pub fn colorize_str(&mut self, text: &str, out: &mut impl Write) -> std::io::Result<()> {
        let mut escaping = false;
        for grapheme in UnicodeSegmentation::graphemes(text, true) {
            escaping = self.handle_grapheme(out, grapheme, escaping)?;
        }

        out.write_all(b"\x1B[39m")?;
        out.flush()
    }

    pub fn colorize_read(
        &mut self,
        input: &mut impl BufRead,
        out: &mut impl Write,
    ) -> std::io::Result<()> {
        input.for_byte_line_with_terminator(|ref line| {
            self.colorize(line, out)?;
            Ok(true)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_rb() -> Rainbow {
        Rainbow::new(
            &RGB {
                r: 0xf0,
                g: 0,
                b: 0,
            },
            (1. / 360. * u32::MAX as f64) as i32,
            (2. / 360. * u32::MAX as f64) as i32,
        )
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
