#![allow(unused_imports, unused_variables, dead_code, unreachable_code)]

use lolcrab::Lolcrab;
use std::io::{self, BufReader};

const TEXT: &str = "\
Vestibulum ante ipsum primis in faucibus orci luctus et
ultrices posuere cubilia curae; Pellentesque at tellus
vitae massa hendrerit hendrerit. Mauris laoreet lectus
quis metus suscipit dignissim. Aliquam magna libero,
lacinia eu justo placerat, gravida aliquam urna.
Pellentesque sodales turpis diam, elementum
sollicitudin lacus rhoncus nec. Pellentesque id dictum
orci, sed laoreet ligula. Nam nec justo eget neque
luctus hendrerit non et diam. Integer eu mi in nisl
imperdiet viverra. Etiam vestibulum neque id posuere
laoreet. Nunc mauris libero, bibendum in vehicula ac,
consectetur a felis. Aenean eu massa vitae sem aliquam
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut lol = Lolcrab::new(None, None);

    println!("\n--- default\n");
    for i in 0..5 {
        lol.set_invert(i % 2 == 1);
        lol.colorize_str(TEXT, &mut stdout)?;
    }

    println!("\n--- reset_position()\n");
    for i in 0..5 {
        lol.set_invert(i % 2 == 1);
        lol.reset_position();
        lol.colorize_str(TEXT, &mut stdout)?;
    }

    println!("\n--- randomize_position()\n");
    lol.gradient = Box::new(colorgrad::preset::viridis());
    lol.set_invert(true);
    for i in 0..7 {
        lol.randomize_position();
        lol.colorize_str(TEXT, &mut stdout)?;
    }

    println!("\n--- animate\n");
    lol.gradient = Box::new(colorgrad::preset::sinebow());
    lol.set_invert(false);
    lol.colorize_read_anim(&mut BufReader::new(TEXT.as_bytes()), &mut stdout)?;
    Ok(())
}
