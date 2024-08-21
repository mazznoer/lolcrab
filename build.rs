#[cfg(feature = "cli")]
include!("src/cli.rs");

#[cfg(feature = "cli")]
fn main() -> Result<(), clap::Error> {
    use clap::CommandFactory;
    use clap_complete::{generate_to, Shell};
    use std::{env, fs, path};

    let outdir = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    if !outdir.exists() {
        fs::create_dir(outdir.clone()).expect("Failed to create 'completions' directory.");
    }

    let mut cmd = Opt::command();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "lolcrab", outdir.clone())?;
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {}
