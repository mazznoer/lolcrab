use std::io::{prelude::*, Write};
use std::{thread, time};

use bstr::{io::BufReadExt, ByteSlice};
use colorgrad::Color;
use noise::NoiseFn;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

#[cfg(feature = "cli")]
use std::process;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "cli")]
pub use cli::*;

pub struct Lolcrab {
    pub gradient: Box<dyn colorgrad::Gradient>,
    pub noise: noise::OpenSimplex,
    noise_scale: f64,
    invert: bool,
    anim_duration: usize,
    anim_sleep: time::Duration,
    x: isize,
    y: isize,
}

impl Lolcrab {
    #[must_use]
    pub fn new(
        gradient: Option<Box<dyn colorgrad::Gradient>>,
        ns: Option<noise::OpenSimplex>,
    ) -> Self {
        Self {
            gradient: gradient.unwrap_or(Box::new(colorgrad::preset::rainbow())),
            noise: ns.unwrap_or(noise::OpenSimplex::new(fastrand::u32(..))),
            noise_scale: 0.034,
            invert: false,
            anim_duration: 5,
            anim_sleep: time::Duration::from_millis(150),
            x: 0,
            y: 0,
        }
    }

    pub fn set_noise_scale(&mut self, scale: f64) {
        self.noise_scale = scale;
    }

    pub fn set_invert(&mut self, invert: bool) {
        self.invert = invert;
    }

    pub fn set_anim_speed(&mut self, speed: u8) {
        self.anim_sleep = time::Duration::from_millis(speed.clamp(30, 200) as u64);
    }

    pub fn set_anim_duration(&mut self, duration: usize) {
        self.anim_duration = duration.clamp(1, 30);
    }

    pub fn step_col(&mut self, n_col: isize) {
        self.x += n_col;
    }

    pub fn step_row(&mut self, n_row: isize) {
        self.y += n_row;
    }

    pub fn reset_col(&mut self) {
        self.x = 0;
    }

    pub fn reset_pos(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn get_color(&mut self) -> Color {
        let position = self.noise.get([
            self.x as f64 * self.noise_scale,
            self.y as f64 * self.noise_scale * 2.0,
        ]) as f32;
        self.gradient.at(remap(position, -0.5, 0.5, -0.1, 1.1))
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
                !c.is_ascii_lowercase() && !c.is_ascii_uppercase()
            };
        } else {
            let col = self.get_color();
            let [r, g, b, _] = col.to_rgba8();

            if self.invert {
                let lum = color_luminance(&col);
                let eps = 0.013;

                let v = if lum < eps {
                    remap(lum, eps, 0.0, 0.22, 0.2)
                } else {
                    remap(lum, eps, 1.0, 0.0, 0.7)
                };
                let [x, y, z, _] = Color::new(v, v, v, 1.0).to_rgba8();

                write!(out, "\x1B[48;2;{r};{g};{b};38;2;{x};{y};{z}m{grapheme}")?;
            } else {
                write!(out, "\x1B[38;2;{r};{g};{b}m{grapheme}")?;
            }

            self.step_col(
                grapheme
                    .chars()
                    .next()
                    .and_then(UnicodeWidthChar::width)
                    .unwrap_or(0) as isize,
            );
        }
        Ok(escaping)
    }

    fn colorize_anim(&mut self, text: &[u8], out: &mut impl Write) -> std::io::Result<()> {
        let mut text_len = 0;
        for g in text.graphemes() {
            text_len += g
                .chars()
                .next()
                .and_then(UnicodeWidthChar::width)
                .unwrap_or(0);
        }
        self.x = -(self.anim_duration as isize - 1) * text_len as isize;
        for _ in 0..self.anim_duration {
            write!(out, "\x1B[{}D", text.len())?;
            self.colorize(text, out)?;
            thread::sleep(self.anim_sleep);
        }
        writeln!(out)?;
        self.reset_col();
        self.step_row(1);
        out.flush()
    }

    /// # Errors
    ///
    /// Will return `Err` if `out` causes I/O erros
    pub fn colorize(&mut self, text: &[u8], out: &mut impl Write) -> std::io::Result<()> {
        let mut escaping = false;
        for grapheme in text.graphemes() {
            escaping = self.handle_grapheme(out, grapheme, escaping)?;
        }

        if self.invert {
            out.write_all(b"\x1B[39;49m")?;
        } else {
            out.write_all(b"\x1B[39m")?;
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

        if self.invert {
            out.write_all(b"\x1B[39;49m")?;
        } else {
            out.write_all(b"\x1B[39m")?;
        }
        out.flush()
    }

    /// # Errors
    ///
    /// Will return `Err` if `input` or `out` cause I/O errors
    pub fn colorize_read_anim(
        &mut self,
        input: &mut impl BufRead,
        out: &mut impl Write,
    ) -> std::io::Result<()> {
        out.write_all(b"\x1B[?25l")?;

        input.for_byte_line(|line| {
            self.colorize_anim(line, out)?;
            Ok(true)
        })?;

        out.write_all(b"\x1B[?25h")?;
        Ok(())
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

#[cfg(feature = "cli")]
impl From<Opt> for Lolcrab {
    fn from(cmd: Opt) -> Self {
        if let Some(seed) = cmd.seed {
            fastrand::seed(seed);
        }

        let grad: Box<dyn colorgrad::Gradient> = if let Some(ref css_grad) = cmd.custom {
            Box::new(
                colorgrad::GradientBuilder::new()
                    .css(css_grad)
                    .mode(colorgrad::BlendMode::Oklab)
                    .build::<colorgrad::CatmullRomGradient>()
                    .unwrap_or_else(|e| {
                        println!("Error: {e}");
                        process::exit(1);
                    }),
            )
        } else if cmd.random_colors.is_some() {
            let n = cmd.random_colors.unwrap();
            let colors = (0..n).map(|_| random_color()).collect::<Vec<_>>();
            Box::new(
                colorgrad::GradientBuilder::new()
                    .colors(&colors)
                    .mode(colorgrad::BlendMode::Oklab)
                    .build::<colorgrad::CatmullRomGradient>()
                    .unwrap(),
            )
        } else {
            match cmd.gradient {
                Gradient::Cividis => Box::new(colorgrad::preset::cividis()),
                Gradient::Cool => Box::new(colorgrad::preset::cool()),
                Gradient::Cubehelix => Box::new(colorgrad::preset::cubehelix_default()),
                Gradient::Inferno => Box::new(colorgrad::preset::inferno()),
                Gradient::Magma => Box::new(colorgrad::preset::magma()),
                Gradient::Plasma => Box::new(colorgrad::preset::plasma()),
                Gradient::Rainbow => Box::new(colorgrad::preset::rainbow()),
                Gradient::RdYlGn => Box::new(colorgrad::preset::rd_yl_gn()),
                Gradient::Sinebow => Box::new(colorgrad::preset::sinebow()),
                Gradient::Spectral => Box::new(colorgrad::preset::spectral()),
                Gradient::Turbo => Box::new(colorgrad::preset::turbo()),
                Gradient::Viridis => Box::new(colorgrad::preset::viridis()),
                Gradient::Warm => Box::new(colorgrad::preset::warm()),
                Gradient::Fruits => build_gradient(&[
                    "#00c21c", "#009dc9", "#ffd43e", "#ff2a70", "#b971ff", "#7ce300", "#feff62",
                ]),
            }
        };

        let grad = if let Some(n) = cmd.sharp {
            if n > 1 {
                Box::new(grad.sharp(n as u16, 0.15))
            } else {
                grad
            }
        } else {
            grad
        };

        let mut lol = Self::new(Some(grad), None);
        lol.set_noise_scale(cmd.scale);
        lol.set_invert(cmd.invert);
        if let Some(speed) = cmd.speed {
            lol.set_anim_speed(speed);
        }
        if let Some(duration) = cmd.duration {
            lol.set_anim_duration(duration as usize);
        }
        lol
    }
}

#[cfg(feature = "cli")]
fn random_color() -> Color {
    if fastrand::bool() {
        Color::from_hwba(fastrand::f32() * 360.0, fastrand::f32() * 0.5, 0.0, 1.0)
    } else {
        Color::from_hwba(fastrand::f32() * 360.0, 0.0, fastrand::f32() * 0.3, 1.0)
    }
}

#[cfg(feature = "cli")]
fn build_gradient(colors: &[&str]) -> Box<dyn colorgrad::Gradient> {
    Box::new(
        colorgrad::GradientBuilder::new()
            .html_colors(colors)
            .mode(colorgrad::BlendMode::Oklab)
            .build::<colorgrad::CatmullRomGradient>()
            .unwrap(),
    )
}

// Reference http://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
fn color_luminance(col: &Color) -> f32 {
    fn lum(t: f32) -> f32 {
        if t <= 0.03928 {
            t / 12.92
        } else {
            ((t + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * lum(col.r) + 0.7152 * lum(col.g) + 0.0722 * lum(col.b)
}

// Map t from range [a, b] to range [c, d]
fn remap(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    (t - a) * ((d - c) / (b - a)) + c
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_rb() -> Lolcrab {
        fastrand::seed(0);
        Lolcrab::new(None, None)
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
        assert_eq!(rb_a.x, 1);

        let test = "😃";
        let mut rb_b = create_rb();
        rb_b.colorize_str(&test, &mut Vec::new()).unwrap();
        assert_eq!(rb_b.x, 2);
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
