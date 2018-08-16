# lolcat-rs
lolcat - in rust! Full unicode support and (optional) escapes for ANSI escape sequences.

## Usage
```
Terminal rainbows.

USAGE:
    lolcat [FLAGS] [OPTIONS] [input]

FLAGS:
    -A, --skip-ansi              Don't evalute ANSI sequences in input
    -h, --help                   Prints help information
    -S, --disable-random-sign    Disable random sign for column and row shift
    -V, --version                Prints version information

OPTIONS:
    -H, --hue <hue>                      Set inital hue (in degrees) [default: random]
    -l, --lighness <lightness>           Lighness of colors in rainbow [default: 0.5]
    -s, --saturation <saturation>        Saturation of colors in rainbow [default: 1.0]
    -c, --shift-column <shift_column>    How much to shift color for every column [default: 1.6]
    -r, --shift-row <shift_row>          How much to shift color for every row [default: 2.2]

ARGS:
    <input>    Input file
```