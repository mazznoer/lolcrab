use bstr::{io::BufReadExt, ByteSlice};
use scarlet::{color::XYZColor, prelude::*};
use std::{
    collections::HashMap,
    io::{prelude::*, Write},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

pub struct Rainbow {
    current_row: usize,
    current_col: usize,
    shift_col: i32,
    shift_row: i32,
    base_hue: u32,
    cache: HashMap<u32, (u8, u8, u8)>,
    color: XYZColor,
}

impl Rainbow {
    pub fn new(color: XYZColor, shift_col: i32, shift_row: i32) -> Self {
        Self {
            color,
            shift_col,
            shift_row,
            current_col: 0,
            current_row: 0,
            cache: HashMap::new(),
            base_hue: (color.hue() * (u32::MAX as f64 / 360.)) as u32,
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

    pub fn get_color(&mut self) -> (u8, u8, u8) {
        let mut hue = self.base_hue;

        if self.shift_row >= 0 {
            let (new_hue, _) = hue.overflowing_add(self.current_row as u32 * self.shift_row as u32);
            hue = new_hue;
        } else {
            let (new_hue, _) =
                hue.overflowing_sub(self.current_row as u32 * (-self.shift_row) as u32);
            hue = new_hue;
        }

        if self.shift_col >= 0 {
            let (new_hue, _) = hue.overflowing_add(self.current_col as u32 * self.shift_col as u32);
            hue = new_hue;
        } else {
            let (new_hue, _) =
                hue.overflowing_sub(self.current_col as u32 * (-self.shift_col) as u32);
            hue = new_hue;
        }

        if let Some(out) = self.cache.get(&hue) {
            return *out;
        }

        self.color.set_hue((hue as f64) / (u32::MAX as f64));
        let out = self.color.convert::<RGBColor>().int_rgb_tup();

        self.cache.insert(hue, out);

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
            let (r, g, b) = self.get_color();
            write!(out, "\x1B[38;2;{};{};{}m{}", r, g, b, grapheme)?;
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
            RGBColor::from_hex_code("#f00000").unwrap().convert(),
            (1. * (u32::MAX as f64 / 360.)) as i32,
            (2. * (u32::MAX as f64 / 360.)) as i32,
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
