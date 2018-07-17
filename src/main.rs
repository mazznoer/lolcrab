extern crate rand;
extern crate termcolor;

use std::f64::consts::PI;
use std::io;
use std::io::prelude::*;
use rand::random;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Eq, PartialEq)]
enum EscapeStatus {
    NoEscape,
    Escape,
}

fn rainbow(width: u64, height: u64) -> Color {
    let freq_width: f64 = 0.1;
    let freq_height: f64 = 0.23;

    let position = width as f64 * freq_width + height as f64 * freq_height;

    let red = position.sin() * 127.0 + 128.0;
    let green = (position + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let blue = (position + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
    Color::Rgb(red as u8, green as u8, blue as u8)
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let input = stdin.lock();

    let outstream = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = outstream.lock();

    let mut color_struct = ColorSpec::new();
    let mut line_count = u64::from(random::<u8>());
    let mut char_count = 0;
    let mut escape_state = EscapeStatus::NoEscape;
    for byte in input.bytes().filter_map(|b| b.ok()) {
        if byte == 0x1B {
            escape_state = EscapeStatus::Escape;
        } else if escape_state == EscapeStatus::Escape {
            if (b'a' <= byte && byte <= b'z') || (b'A' <= byte && byte <= b'Z') {
                escape_state = EscapeStatus::NoEscape;
            }
        } else if byte == b'\n' {
            line_count += 1;
            char_count = 0;
        } else {
            let color = rainbow(line_count, char_count);
            color_struct.set_fg(Some(color));
            stdout.set_color(&color_struct)?;
        }

        stdout.write_all(&[byte])?;
    }
    Ok(())
}
