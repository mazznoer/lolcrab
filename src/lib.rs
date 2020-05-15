mod rainbow;
#[cfg(feature = "structopt")]
mod rainbow_cmd;

pub use rainbow::*;

#[cfg(feature = "structopt")]
pub use rainbow_cmd::*;
