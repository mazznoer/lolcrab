use clap::{Parser, ValueEnum};
use colorgrad::{Color, ParseColorError};

use crate::Rainbow;

fn parse_color(s: &str) -> Result<Color, ParseColorError> {
    s.parse::<Color>()
}

#[derive(Debug, Clone, ValueEnum)]
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

fn random_color() -> Color {
    if fastrand::bool() {
        Color::from_hwba(fastrand::f32() * 360.0, fastrand::f32() * 0.5, 0.0, 1.0)
    } else {
        Color::from_hwba(fastrand::f32() * 360.0, 0.0, fastrand::f32() * 0.3, 1.0)
    }
}

#[derive(Debug, Parser)]
pub struct RainbowCmd {
    /// Sets color gradient
    #[arg(
        short,
        long,
        value_enum,
        default_value = "rainbow",
        value_name = "NAME"
    )]
    gradient: Gradient,

    /// Create custom gradient using the specified colors
    #[arg(short = 'c', long, value_parser = parse_color, num_args = 1.., value_name = "COLOR")]
    custom: Option<Vec<Color>>,

    /// Sharp gradient
    #[arg(long, value_name = "NUM")]
    sharp: Option<u8>,

    /// Sets noise scale. Try value between 0.01 .. 0.2
    #[arg(short, long, default_value = "0.034", value_name = "FLOAT")]
    scale: f64,

    /// Sets seed [default: random]
    #[arg(short = 'S', long, value_name = "NUM")]
    seed: Option<u64>,

    /// Colorize the background
    #[arg(short = 'i', long)]
    invert: bool,

    /// Use random colors as custom gradient [1 .. 100]
    #[arg(short = 'r', long, value_name = "NUM", value_parser = clap::value_parser!(u8).range(1..=100))]
    random_colors: Option<u8>,

    /// Enable animation mode
    #[arg(short = 'a', long)]
    animate: bool,

    /// Animation duration
    #[arg(short = 'd', long, value_name = "NUM")]
    duration: Option<u8>,

    /// Animation speed
    #[arg(long)]
    speed: Option<u8>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    RainbowCmd::command().debug_assert()
}

impl From<RainbowCmd> for Rainbow {
    fn from(cmd: RainbowCmd) -> Self {
        if let Some(seed) = cmd.seed {
            fastrand::seed(seed);
        }

        let grad: Box<dyn colorgrad::Gradient> = if let Some(colors) = cmd.custom {
            Box::new(
                colorgrad::GradientBuilder::new()
                    .colors(&colors)
                    .mode(colorgrad::BlendMode::Oklab)
                    .build::<colorgrad::CatmullRomGradient>()
                    .unwrap(),
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

        let duration = cmd.duration.unwrap_or(5) as usize;
        let speed = cmd.speed.unwrap_or(150);
        Self::new(grad, cmd.scale, cmd.invert, cmd.animate, duration, speed)
    }
}

fn build_gradient(colors: &[&str]) -> Box<dyn colorgrad::Gradient> {
    Box::new(
        colorgrad::GradientBuilder::new()
            .html_colors(colors)
            .mode(colorgrad::BlendMode::Oklab)
            .build::<colorgrad::CatmullRomGradient>()
            .unwrap(),
    )
}
