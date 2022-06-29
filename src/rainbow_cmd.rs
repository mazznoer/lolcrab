use clap::{ArgEnum, Parser};
use colorgrad::{Color, ParseColorError};

use crate::Rainbow;

fn parse_color(s: &str) -> Result<Color, ParseColorError> {
    s.parse::<Color>()
}

#[derive(Debug, Clone, ArgEnum)]
pub enum Gradient {
    Cividis,
    Cool,
    Cubehelix,
    Fruits,
    Inferno,
    Magma,
    Plasma,
    Rainbow,
    RdYlGn,
    Sinebow,
    Spectral,
    Turbo,
    Viridis,
    Warm,
}

fn random_colors_validator(s: &str) -> Result<(), String> {
    match s.parse::<u8>() {
        Ok(t) if (1..=100).contains(&t) => Ok(()),
        _ => Err(String::from("Valid value is 1 to 100")),
    }
}

fn random_color() -> Color {
    if fastrand::bool() {
        Color::from_hwb(fastrand::f64() * 360.0, fastrand::f64() * 0.5, 0.0)
    } else {
        Color::from_hwb(fastrand::f64() * 360.0, 0.0, fastrand::f64() * 0.3)
    }
}

#[derive(Debug, Parser)]
pub struct RainbowCmd {
    /// Sets color gradient
    #[clap(short, long, arg_enum, default_value = "rainbow", value_name = "NAME")]
    gradient: Gradient,

    /// Create custom gradient using the specified colors
    #[clap(short = 'c', long, parse(try_from_str = parse_color), multiple_values = true, value_name = "COLOR")]
    custom: Option<Vec<Color>>,

    /// Sharp gradient
    #[clap(long, value_name = "NUM")]
    sharp: Option<u8>,

    /// Sets noise scale. Try value between 0.01 .. 0.2
    #[clap(short, long, default_value = "0.034", value_name = "FLOAT")]
    scale: f64,

    /// Sets seed [default: random]
    #[clap(short = 'S', long, value_name = "NUM")]
    seed: Option<u64>,

    /// Colorize the background
    #[clap(short = 'i', long)]
    invert: bool,

    /// Use random colors as custom gradient [1 .. 100]
    #[clap(short = 'r', long, value_name = "NUM", validator = random_colors_validator)]
    random_colors: Option<u8>,

    /// Enable animation mode
    #[clap(short = 'a', long)]
    animate: bool,

    /// Animation duration
    #[clap(short = 'd', long, value_name = "NUM")]
    duration: Option<u8>,

    /// Animation speed
    #[clap(long)]
    speed: Option<u8>,
}

impl From<RainbowCmd> for Rainbow {
    fn from(cmd: RainbowCmd) -> Self {
        if let Some(seed) = cmd.seed {
            fastrand::seed(seed);
        }

        let grad = if let Some(colors) = cmd.custom {
            colorgrad::CustomGradient::new()
                .colors(&colors)
                .mode(colorgrad::BlendMode::Oklab)
                .interpolation(colorgrad::Interpolation::CatmullRom)
                .build()
                .unwrap()
        } else if cmd.random_colors.is_some() {
            let n = cmd.random_colors.unwrap();
            let colors = (0..n).map(|_| random_color()).collect::<Vec<_>>();
            colorgrad::CustomGradient::new()
                .colors(&colors)
                .mode(colorgrad::BlendMode::Oklab)
                .interpolation(colorgrad::Interpolation::CatmullRom)
                .build()
                .unwrap()
        } else {
            match cmd.gradient {
                Gradient::Cividis => colorgrad::cividis(),
                Gradient::Cool => colorgrad::cool(),
                Gradient::Cubehelix => colorgrad::cubehelix_default(),
                Gradient::Inferno => colorgrad::inferno(),
                Gradient::Magma => colorgrad::magma(),
                Gradient::Plasma => colorgrad::plasma(),
                Gradient::Rainbow => colorgrad::rainbow(),
                Gradient::RdYlGn => colorgrad::rd_yl_gn(),
                Gradient::Sinebow => colorgrad::sinebow(),
                Gradient::Spectral => colorgrad::spectral(),
                Gradient::Turbo => colorgrad::turbo(),
                Gradient::Viridis => colorgrad::viridis(),
                Gradient::Warm => colorgrad::warm(),
                Gradient::Fruits => build_gradient(&[
                    "#00c21c", "#009dc9", "#ffd43e", "#ff2a70", "#b971ff", "#7ce300", "#feff62",
                ]),
            }
        };

        let grad = if let Some(n) = cmd.sharp {
            if n > 1 {
                grad.sharp(n as usize, 0.15)
            } else {
                grad
            }
        } else {
            grad
        };

        let duration = cmd.duration.unwrap_or(5) as usize;
        let speed = cmd.speed.unwrap_or(150);
        Self::new(grad, cmd.scale, cmd.invert, cmd.animate, duration, speed)
    }
}

fn build_gradient(colors: &[&str]) -> colorgrad::Gradient {
    colorgrad::CustomGradient::new()
        .html_colors(colors)
        .mode(colorgrad::BlendMode::Oklab)
        .interpolation(colorgrad::Interpolation::CatmullRom)
        .build()
        .unwrap()
}
