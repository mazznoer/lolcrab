mod rainbow;
#[cfg(feature = "structopt")]
mod rainbow_cmd;

pub use rainbow::Rainbow;

#[cfg(feature = "structopt")]
pub use rainbow_cmd::RainbowCmd;
