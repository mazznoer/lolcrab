# lcat

lolcat in rust! With emoji support and color transformations in the Oklab color space.

## Usage

```text
lcat 
Terminal rainbows.

USAGE:
    lcat [FLAGS] [OPTIONS] [File]...

ARGS:
    <File>...    [default: -]

FLAGS:
        --help                    Prints help information
    -n, --shift-sign-no-random    Don't randomize sign of col and row shift values
    -V, --version                 Prints version information

OPTIONS:
    -c, --chroma <chroma>          Sets text color chroma [default: 0.5]
    -h, --hue <hue>                Sets initial hue of text color in degress [default: random]
    -l, --luminance <luminance>    Sets text color luminance [default: 0.85]
    -C, --shift-col <shift-col>    How many degrees to shift text color hue for every column
                                   [default: 1.6]
    -R, --shift-row <shift-row>    How many degrees to shift text color hue for every row [default:
                                   3.2]```