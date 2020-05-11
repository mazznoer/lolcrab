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

    pub fn reset_row(&mut self) {
        self.step_row(-self.current_row)
    }

    pub fn reset_col(&mut self) {
        self.step_col(-self.current_col)
    }

    pub fn colorize(&mut self, text: &[u8], out: &mut impl Write) -> std::io::Result<()> {
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

        out.write_all(b"\x1B[39m")?;
        out.flush()
    }

    pub fn colorize_str(&mut self, text: &str, out: &mut impl Write) -> std::io::Result<()> {
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

        out.write_all(b"\x1B[39m")?;
        out.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_rb() -> Rainbow {
        Rainbow::new(&RGBColor::from_hex_code("#f00000").unwrap(), 1., 2.)
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
    fn test_step_row()  {
        let test_string = "foobar\n";

        let mut rb_a = create_rb();
        rb_a.colorize(&test_string.as_bytes(), &mut Vec::new()).unwrap();
        let mut rb_b = create_rb();
        rb_b.step_row(1);
        assert_eq!(
            rb_a.color.convert::<RGBColor>().int_rgb_tup(),
            rb_b.color.convert::<RGBColor>().int_rgb_tup()
        );
    }

    #[test]
    fn test_reset_row() {
        let mut rb_a = create_rb();
        let rb_b = create_rb();
        rb_a.step_row(20);
        rb_a.reset_row();
        assert_eq!(
            rb_a.color.convert::<RGBColor>().int_rgb_tup(),
            rb_b.color.convert::<RGBColor>().int_rgb_tup()
        );

    }

    #[test]
    fn test_reset_col() {
        let mut rb_a = create_rb();
        let rb_b = create_rb();
        rb_a.step_col(20);
        rb_a.reset_col();
        assert_eq!(
            rb_a.color.convert::<RGBColor>().int_rgb_tup(),
            rb_b.color.convert::<RGBColor>().int_rgb_tup()
        );
    }
}
