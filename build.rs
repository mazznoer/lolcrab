#[cfg(feature = "cli")]
use clap::CommandFactory;
#[cfg(feature = "cli")]
use clap_complete::{generate_to, Shell};
#[cfg(feature = "cli")]
use std::env;

#[cfg(feature = "cli")]
include!("src/cli.rs");

#[cfg(feature = "cli")]
fn main() -> Result<(), clap::Error> {
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    if !outdir.exists() {
        std::fs::create_dir(outdir.clone()).expect("Failed to create 'completions' directory.");
    }

    let mut cmd = Opt::command();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "lolcrab", outdir.clone())?;
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {}
