use clap::{Parser, ValueEnum};
use std::path;

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

#[derive(Clone, Debug, Parser)]
#[command(
    name = "lolcrab",
    version,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Opt {
    /// Files to read
    #[arg(name = "File", default_value = "-", value_parser = clap::value_parser!(path::PathBuf))]
    pub files: Vec<path::PathBuf>,

    /// Sets color gradient
    #[arg(
        short,
        long,
        value_enum,
        default_value = "rainbow",
        value_name = "NAME"
    )]
    pub gradient: Gradient,

    /// Show all preset gradients
    #[arg(long)]
    pub presets: bool,

    /// Custom gradient in CSS gradient format
    #[arg(short = 'c', long, value_name = "CSS Gradient")]
    pub custom: Option<String>,

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

    /// Print help
    #[arg(short = 'h', long)]
    pub help: bool,

    /// Print version
    #[arg(short = 'V', long)]
    pub version: bool,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Opt::command().debug_assert()
}
