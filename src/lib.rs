#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::cast_precision_loss)]

mod rainbow;
#[cfg(feature = "clap")]
mod rainbow_cmd;

pub use rainbow::*;

#[cfg(feature = "clap")]
pub use rainbow_cmd::*;
