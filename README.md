# lcat

Another lolcat in rust! Full unicode support, escapes for ANSI escape sequences, hue shift in CIE L*C*h.

## Usage

```text
Terminal rainbows.

USAGE:
    lcat [FLAGS] [OPTIONS] [File]...

FLAGS:
        --help                    Prints help information
    -n, --shift-sign-no-random    Randomize sign of col and row shift value
    -V, --version                 Prints version information

OPTIONS:
    -c, --chroma <chroma>          Sets initial chroma as defined by CIE L*C*h Color Scale [default: 128]
    -h, --hue <hue>                Sets initial hue as defined by CIE L*C*h Color Scale [default: random]
    -l, --luminance <luminance>    Sets initial luminance as defined by CIE L*C*h Color Scale [default: 50]
    -C, --shift-col <shift-col>    How much the hue of the color gets shifted every column [default: 1.6]
    -R, --shift-row <shift-row>    How much the hue of the color gets shifted every row [default: 3.2]

ARGS:
    <File>...     [default: -]
```