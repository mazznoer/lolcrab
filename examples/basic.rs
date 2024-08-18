use lolcrab::Lolcrab;
use std::io;

const TEXT: &str = "\
•••••••••••••••••••••••••••••••••••••••••••
••442463299144744830108724702438783348716••
••665891426009540978622724448305819269356••
••078289454141226451790882961903610719673••
••56505384476•••••••••••••••••39761609699••
••47928752907•• { lolcrab } ••33810561851••
••51609982385•••••••••••••••••43459368213••
••980457234663167653959566555465520046709••
••677103598707232478714861999441705454744••
••012721882924436718718457599087686681354••
•••••••••••••••••••••••••••••••••••••••••••
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Initialize Lolcrab using default gradient and default noise
    let mut lol = Lolcrab::new(None, None);

    lol.colorize_str(TEXT, &mut stdout)?;

    lol.set_invert(true);
    lol.randomize_position();
    lol.colorize_str(TEXT, &mut stdout)?;

    lol.set_invert(false);
    lol.reset_position();
    lol.colorize_str(TEXT, &mut stdout)?;

    Ok(())
}
