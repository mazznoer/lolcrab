#[cfg(feature = "cli")]
include!("src/cli.rs");

#[cfg(feature = "cli")]
fn main() -> Result<(), clap::Error> {
    use clap::CommandFactory;
    use clap_complete::{generate_to, Shell};
    use std::{fs, path, process::exit};

    let out_dir = option_env!("OUT_DIR").unwrap_or_else(|| {
        exit(0);
    });
    let dir = path::Path::new(&out_dir).join("completions/");
    if !dir.exists() {
        fs::create_dir(dir.clone()).expect("Failed to create 'completions' directory.");
    }

    let mut cmd = Opt::command();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "lolcrab", dir.clone())?;
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {}
