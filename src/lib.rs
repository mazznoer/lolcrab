//! # `Lolcrab`
//!
//! Like [`lolcat`](https://github.com/busyloop/lolcat) but with [noise](https://en.wikipedia.org/wiki/OpenSimplex_noise) and more colorful.
//!
//! ## Using `lolcrab` as a Library
//!
//! Add this to your Cargo.toml
//!
//! ```toml
//! lolcrab = { version = "0.4", default-features = "false" }
//! ```
//!

use std::f32::consts::TAU;
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
pub use cli::{Gradient, Opt};

/// # Example
///
/// ```
/// # use std::error::Error;
/// use lolcrab::Lolcrab;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let stdout = std::io::stdout();
/// let mut stdout = stdout.lock();
///
/// // Initialize Lolcrab using default gradient and default noise
/// let mut lol = Lolcrab::new(None, None);
///
/// lol.colorize_str("Lolcrab is the best", &mut stdout)?;
///
/// lol.set_invert(true);
/// lol.randomize_position();
/// lol.colorize_str("Lolcrab is the best", &mut stdout)?;
/// # Ok(())
/// # }
/// ```
pub struct Lolcrab {
    pub gradient: Box<dyn colorgrad::Gradient>,
    pub noise: Box<dyn noise::NoiseFn<f64, 2>>,
    noise_scale: f64,
    invert: bool,
    tab_width: isize,
    x: isize,
    y: isize,

    linear: bool,
    shift_x: f32,
    shift_y: f32,
    offset: f32,
    angle: f32,
    distance: f32,

    anim_duration: usize,
    anim_sleep: time::Duration,
}

impl Lolcrab {
    #[must_use]
    pub fn new(
        gradient: Option<Box<dyn colorgrad::Gradient>>,
        ns: Option<Box<dyn noise::NoiseFn<f64, 2>>>,
    ) -> Self {
        let angle = fastrand::f32() * TAU;
        let distance = 0.017;
        Self {
            gradient: gradient.unwrap_or(Box::new(colorgrad::preset::rainbow())),
            noise: ns.unwrap_or(Box::new(noise::OpenSimplex::new(fastrand::u32(..)))),
            noise_scale: 0.034,
            invert: false,
            tab_width: 4,
            x: 0,
            y: 0,

            linear: false,
            shift_x: angle.cos() * distance / 2.0,
            shift_y: angle.sin() * distance,
            offset: fastrand::f32(),
            angle,
            distance,

            anim_duration: 5,
            anim_sleep: time::Duration::from_millis(150),
        }
    }

    /// Noise scale. Try value between 0.01 .. 0.2
    pub fn set_noise_scale(&mut self, scale: f64) {
        self.noise_scale = scale.clamp(0.001, 0.25);
    }

    /// Colorize the background if set to true
    pub fn set_invert(&mut self, invert: bool) {
        self.invert = invert;
    }

    /// Tab stop width (default: 4)
    pub fn set_tab_width(&mut self, width: usize) {
        self.tab_width = width as isize;
    }

    /// Animation speed (30..200)
    pub fn set_anim_speed(&mut self, speed: u8) {
        self.anim_sleep = time::Duration::from_millis(speed.clamp(30, 200) as u64);
    }

    /// Animation duration (1..30)
    pub fn set_anim_duration(&mut self, duration: usize) {
        self.anim_duration = duration.clamp(1, 30);
    }

    /// Linear mode
    pub fn set_linear(&mut self, b: bool) {
        self.linear = b;
    }

    fn calc_shift(&mut self) {
        self.shift_x = self.angle.cos() * self.distance / 2.0;
        self.shift_y = self.angle.sin() * self.distance;
    }

    /// Sets angle in degrees (0..360)
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle.to_radians();
        self.calc_shift();
    }

    /// Sets spread (0..100)
    pub fn set_spread(&mut self, distance: f32) {
        self.distance = remap(distance, 0.0, 100.0, 0.005, 0.1);
        self.calc_shift();
    }

    /// Sets color gradient offset (0..1)
    pub fn set_offset(&mut self, offset: f32) {
        self.offset = offset.clamp(0.0, 1.0);
    }

    #[doc(hidden)]
    pub fn step_col(&mut self, n_col: isize) {
        self.x += n_col;
    }

    #[doc(hidden)]
    pub fn step_row(&mut self, n_row: isize) {
        self.y += n_row;
    }

    #[doc(hidden)]
    pub fn reset_col(&mut self) {
        self.x = 0;
    }

    /// Reset noise position
    pub fn reset_position(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    /// Randomize noise position
    pub fn randomize_position(&mut self) {
        self.x = 0;
        self.y = fastrand::isize(-999_999..999_999);
    }

    #[doc(hidden)]
    pub fn get_color(&mut self) -> Color {
        if self.linear {
            let t = self.offset + self.x as f32 * self.shift_x + self.y as f32 * self.shift_y;
            return self.gradient.at(modulo(t, 1.0));
        }
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

    // TODO
    fn colorize_anim(&mut self, text: &[u8], out: &mut impl Write) -> std::io::Result<()> {
        let mut text_len: isize = 0;
        for g in text.graphemes() {
            if g == "\t" {
                text_len += self.tab_width - text_len % self.tab_width;
            } else {
                text_len += g
                    .chars()
                    .next()
                    .and_then(UnicodeWidthChar::width)
                    .unwrap_or(0) as isize;
            }
        }
        self.x = -(self.anim_duration as isize - 1) * text_len;
        for _ in 0..self.anim_duration {
            out.write_all(b"\x1B[0G")?;
            self.colorize(text, out)?;
            thread::sleep(self.anim_sleep);
        }
        out.write_all(b"\n")?;
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
            if grapheme == "\t" {
                let n = self.tab_width - self.x % self.tab_width;
                if self.invert {
                    for _ in 0..n {
                        escaping = self.handle_grapheme(out, " ", escaping)?;
                    }
                } else {
                    self.step_col(n);
                    out.write_all(" ".repeat(n as usize).as_bytes())?;
                }
            } else {
                escaping = self.handle_grapheme(out, grapheme, escaping)?;
            }
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
            if grapheme == "\t" {
                let n = self.tab_width - self.x % self.tab_width;
                if self.invert {
                    for _ in 0..n {
                        escaping = self.handle_grapheme(out, " ", escaping)?;
                    }
                } else {
                    self.step_col(n);
                    out.write_all(" ".repeat(n as usize).as_bytes())?;
                }
            } else {
                escaping = self.handle_grapheme(out, grapheme, escaping)?;
            }
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
        // hide the cursor
        out.write_all(b"\x1B[?25l")?;

        input.for_byte_line(|line| {
            self.colorize_anim(line, out)?;
            Ok(true)
        })?;

        // show the cursor
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
        } else if cmd.lolcat {
            Gradient::Lolcat.to_gradient()
        } else {
            cmd.gradient.to_gradient()
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
        if let Some(angle) = cmd.angle {
            lol.set_linear(true);
            lol.set_angle(angle);
        }
        if let Some(spread) = cmd.spread {
            lol.set_linear(true);
            lol.set_spread(spread);
        }
        if let Some(offset) = cmd.offset {
            lol.set_linear(true);
            lol.set_offset(offset);
        }
        if cmd.linear || cmd.lolcat {
            lol.set_linear(true);
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

#[inline]
fn modulo(x: f32, y: f32) -> f32 {
    (x % y + y) % y
}

#[cfg(test)]
mod tests {
    use super::Lolcrab;

    fn new_lol(seed: u32) -> Lolcrab {
        Lolcrab::new(None, Some(Box::new(noise::OpenSimplex::new(seed))))
    }

    #[test]
    fn eq_str_u8() {
        let text = "foobar";

        let mut lol1 = new_lol(7);
        let mut out1 = Vec::new();
        lol1.colorize(&text.as_bytes(), &mut out1).unwrap();

        let mut lol2 = new_lol(7);
        let mut out2 = Vec::new();
        lol2.colorize_str(&text, &mut out2).unwrap();

        assert_eq!(out1, out2);
    }

    #[test]
    fn str_width() {
        let mut lol = Lolcrab::new(None, None);
        let mut out = Vec::new();

        lol.colorize_str("f", &mut out).unwrap();
        assert_eq!(lol.x, 1);

        lol.reset_col();
        lol.colorize_str("😃", &mut out).unwrap();
        assert_eq!(lol.x, 2);

        lol.reset_col();
        lol.colorize_str(" ", &mut out).unwrap();
        assert_eq!(lol.x, 1);

        lol.reset_col();
        lol.colorize_str("  ", &mut out).unwrap();
        assert_eq!(lol.x, 2);

        // Tab characters

        lol.reset_col();
        lol.colorize_str("\t", &mut out).unwrap();
        assert_eq!(lol.x, 4);

        lol.reset_col();
        lol.colorize_str(" \t", &mut out).unwrap();
        assert_eq!(lol.x, 4);

        lol.reset_col();
        lol.colorize_str("  \t", &mut out).unwrap();
        assert_eq!(lol.x, 4);

        lol.reset_col();
        lol.colorize_str("   \t", &mut out).unwrap();
        assert_eq!(lol.x, 4);

        lol.reset_col();
        lol.set_tab_width(8);
        lol.colorize_str("   \t", &mut out).unwrap();
        assert_eq!(lol.x, 8);

        lol.reset_col();
        lol.set_tab_width(4);
        lol.colorize_str("    \t", &mut out).unwrap();
        assert_eq!(lol.x, 8);

        lol.reset_col();
        lol.colorize_str("\t  ", &mut out).unwrap();
        assert_eq!(lol.x, 6);

        lol.reset_col();
        lol.colorize_str("\t  \t", &mut out).unwrap();
        assert_eq!(lol.x, 8);
    }

    #[test]
    fn step_row() {
        let text = "foobar\n";

        let mut lol1 = new_lol(0);
        lol1.colorize(&text.as_bytes(), &mut Vec::new()).unwrap();

        let mut lol2 = new_lol(0);
        lol2.step_row(1);

        assert_eq!(lol1.x, lol2.x);
        assert_eq!(lol1.y, lol2.y);
        assert_eq!(lol1.get_color().to_rgba8(), lol2.get_color().to_rgba8());
    }

    #[test]
    fn reset_col() {
        let mut lol1 = new_lol(23);
        let mut lol2 = new_lol(23);
        lol1.step_col(20);
        lol1.reset_col();
        assert_eq!(lol1.get_color().to_rgba8(), lol2.get_color().to_rgba8());
    }

    #[test]
    fn noise_position() {
        let mut lol = Lolcrab::new(None, None);
        let text = "Lolcrab\nRust";

        let mut out1 = Vec::new();
        lol.colorize_str(text, &mut out1).unwrap();

        let mut out2 = Vec::new();
        lol.colorize_str(text, &mut out2).unwrap();

        let mut out3 = Vec::new();
        lol.reset_position();
        lol.colorize_str(text, &mut out3).unwrap();

        let mut out4 = Vec::new();
        lol.randomize_position();
        lol.colorize_str(text, &mut out4).unwrap();

        assert_ne!(out1, out2);
        assert_eq!(out1, out3);
        assert_ne!(out1, out4);
        assert_ne!(out2, out4);
    }
}
