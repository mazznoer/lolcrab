use crate::Rainbow;
use clap::{ArgEnum, Parser};
use colorgrad::{Color, ParseColorError};

fn parse_color(s: &str) -> Result<Color, ParseColorError> {
    s.parse::<Color>()
}

#[derive(Debug, Clone, ArgEnum)]
pub enum Gradient {
    Cividis,
    Cool,
    Cubehelix,
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
}

impl From<RainbowCmd> for Rainbow {
    fn from(cmd: RainbowCmd) -> Self {
        let grad = if let Some(colors) = cmd.custom {
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

        Self::new(grad, cmd.seed, cmd.scale, cmd.invert)
    }
}
