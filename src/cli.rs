use clap::{Parser, ValueEnum};
use colorgrad::Color;

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

#[derive(Debug, Parser)]
pub struct Opt {
    /// Sets color gradient
    #[arg(
        short,
        long,
        value_enum,
        default_value = "rainbow",
        value_name = "NAME"
    )]
    pub gradient: Gradient,

    /// Create custom gradient using the specified colors
    #[arg(short = 'c', long, num_args = 1.., value_name = "COLOR")]
    pub custom: Option<Vec<Color>>,

    /// Sharp gradient
    #[arg(long, value_name = "NUM")]
    pub sharp: Option<u8>,

    /// Sets noise scale. Try value between 0.01 .. 0.2
    #[arg(short, long, default_value = "0.034", value_name = "FLOAT")]
    pub scale: f64,

    /// Sets seed [default: random]
    #[arg(short = 'S', long, value_name = "NUM")]
    pub seed: Option<u64>,

    /// Colorize the background
    #[arg(short = 'i', long)]
    pub invert: bool,

    /// Use random colors as custom gradient [1 .. 100]
    #[arg(short = 'r', long, value_name = "NUM", value_parser = clap::value_parser!(u8).range(1..=100))]
    pub random_colors: Option<u8>,

    /// Enable animation mode
    #[arg(short = 'a', long)]
    pub animate: bool,

    /// Animation duration
    #[arg(short = 'd', long, value_name = "NUM")]
    pub duration: Option<u8>,

    /// Animation speed
    #[arg(long)]
    pub speed: Option<u8>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Opt::command().debug_assert()
}
