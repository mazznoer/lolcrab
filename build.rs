use clap::*;
use clap_complete::{generate_to, Shell};
use std::env;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
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
