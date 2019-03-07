use crate::{state::State, Cmdline};
use bytes::BytesMut;
use tokio::codec::Encoder;
use unicode_segmentation::UnicodeSegmentation;
pub struct LolCodec {
    rainbow_state: State,
    skip_ansi: bool,
}

impl LolCodec {
    pub fn new(opts: &Cmdline) -> LolCodec {
        LolCodec {
            rainbow_state: State::from_opts(&opts.lol_options),
            skip_ansi: opts.skip_ansi,
        }
    }
}

impl Encoder for LolCodec {
    type Item = String;
    type Error = std::io::Error;

    fn encode(&mut self, line: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut print_color = true;
        UnicodeSegmentation::graphemes(&line[..], true).for_each(|grapheme| {
            if grapheme == "\x1B" {
                print_color = false;
            }

            if print_color || self.skip_ansi {
                let [r, g, b] = self.rainbow_state.feed(&grapheme);
                let out = format!("\x1B[38;2;{};{};{}m{}", r, g, b, grapheme);
                let out = out.as_bytes();
                dst.extend_from_slice(out);
            } else {
                if "a" <= grapheme && "z" >= grapheme || "A" <= grapheme && "Z" >= grapheme {
                    print_color = true;
                }
                let out = grapheme.as_bytes();
                dst.extend_from_slice(out);
            }
        });
        dst.extend_from_slice(b"\x1B[0m\n");
        self.rainbow_state.bump_line();
        Ok(())
    }
}
