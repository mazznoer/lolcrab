use std::f64::consts::{PI, TAU};
#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct Lab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct LCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn to_linear_srgb(x: u8) -> f64 {
    let x = (x as f64) / 255.0;
    if x >= 0.0031308 {
        (1.055) * x.powf(1.0 / 2.4) - 0.055
    } else {
        12.92 * x
    }
}

fn from_linear_srgb(x: f64) -> u8 {
    let y = if x >= 0.04045 {
        ((x + 0.055) / (1.0 + 0.055)).powf(2.4)
    } else {
        x / 12.92
    };
    (y * 255.0).round() as u8
}

pub trait Hue {
    fn hue(&self) -> f64;
    fn set_hue(&mut self, hue: f64);
    fn set_hue_with_chroma(&mut self, hue: f64, _chroma: f64) {
        self.set_hue(hue);
    }
}

impl Hue for LCh {
    fn hue(&self) -> f64 {
        self.h
    }

    fn set_hue(&mut self, hue: f64) {
        self.h = hue;
    }
}

pub fn hue_as_u32(hue: f64) -> u32 {
    ((hue + PI) / TAU * (u32::MAX as f64)) as u32
}

pub fn hue_as_f64(hue: u32) -> f64 {
    (hue as f64) / (u32::MAX as f64) * TAU - PI
}

impl Lab {
    pub fn chroma(&self) -> f64 {
        self.a.hypot(self.b)
    }
}

impl Hue for Lab {
    fn hue(&self) -> f64 {
        self.b.atan2(self.a)
    }

    fn set_hue(&mut self, hue: f64) {
        self.set_hue_with_chroma(hue, self.chroma());
    }

    fn set_hue_with_chroma(&mut self, hue: f64, chroma: f64) {
        let (hue_sin, hue_cos) = hue.sin_cos();

        self.a = chroma * hue_sin;
        self.b = chroma * hue_cos;
    }
}

impl From<&Lab> for LCh {
    fn from(color: &Lab) -> Self {
        Self {
            L: color.L,
            C: color.chroma(),
            h: color.hue(),
        }
    }
}

impl From<&LCh> for Lab {
    fn from(color: &LCh) -> Self {
        let (hue_sin, hue_cos) = color.h.sin_cos();
        Self {
            L: color.L,
            a: color.C * hue_cos,
            b: color.C * hue_sin,
        }
    }
}

impl From<&Lab> for RGB {
    #[allow(clippy::many_single_char_names)]
    fn from(color: &Lab) -> Self {
        let l_ = color.L + 0.3963377774 * color.a + 0.2158037573 * color.b;
        let m_ = color.L - 0.1055613458 * color.a - 0.0638541728 * color.b;
        let s_ = color.L - 0.0894841775 * color.a - 1.2914855480 * color.b;

        let l = l_.powi(3);
        let m = m_.powi(3);
        let s = s_.powi(3);

        Self {
            r: from_linear_srgb(4.0767245293 * l - 3.3072168827 * m + 0.2307590544 * s),
            g: from_linear_srgb(-1.2681437731 * l + 2.6093323231 * m - 0.3411344290 * s),
            b: from_linear_srgb(-0.0041119885 * l - 0.7034763098 * m + 1.7068625689 * s),
        }
    }
}

impl From<&LCh> for RGB {
    fn from(color: &LCh) -> Self {
        RGB::from(&Lab::from(color))
    }
}

impl From<&RGB> for LCh {
    fn from(color: &RGB) -> Self {
        LCh::from(&Lab::from(color))
    }
}

impl From<&RGB> for Lab {
    #[allow(clippy::many_single_char_names)]
    fn from(color: &RGB) -> Self {
        let r = to_linear_srgb(color.r);
        let g = to_linear_srgb(color.g);
        let b = to_linear_srgb(color.b);

        let l_ = 0.4121656120 * r + 0.5362752080 * g + 0.0514575653 * b;
        let m_ = 0.2118591070 * r + 0.6807189584 * g + 0.1074065790 * b;
        let s_ = 0.0883097947 * r + 0.2818474174 * g + 0.6302613616 * b;

        let l = l_.cbrt();
        let m = m_.cbrt();
        let s = s_.cbrt();

        Lab {
            L: 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
            a: 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
            b: 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_trip() {
        let c_rgb1 = RGB {
            r: 80,
            g: 160,
            b: 240,
        };

        let c_lch = LCh::from(&c_rgb1);
        let c_rgb2 = RGB::from(&c_lch);

        assert_eq!(c_rgb1, c_rgb2);
    }

    #[test]
    fn test_hue() {
        let c_rgb = RGB {
            r: 80,
            g: 160,
            b: 240,
        };

        let mut c_lch = LCh::from(&c_rgb);

        let h = c_lch.hue_u32();

        let a = RGB::from(&c_lch);
        c_lch.set_hue_u32(h);
        let b = RGB::from(&c_lch);

        assert_eq!(a, b);

        c_lch.set_hue(0.0);
        let a = RGB::from(&c_lch);
        c_lch.set_hue_u32(u32::MAX / 2);
        let b = RGB::from(&c_lch);
        assert_eq!(a, b);
    }
}
