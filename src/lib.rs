#![allow(clippy::cast_precision_loss)]

#[cfg(feature = "clap")]
mod cli;
mod rainbow;

pub use rainbow::*;

#[cfg(feature = "clap")]
pub use cli::*;
