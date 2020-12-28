mod color;
mod rainbow;
#[cfg(feature = "clap")]
mod rainbow_cmd;

pub use color::*;
pub use rainbow::*;

#[cfg(feature = "clap")]
pub use rainbow_cmd::*;
