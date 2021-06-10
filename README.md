# lcat

lolcat in rust! With emoji support and color transformations in the Cubehelix color space.

## Usage

```text
lcat
Terminal rainbows.

USAGE:
    lcat [FLAGS] [OPTIONS] [File]...

ARGS:
    <File>...    [default: -]

FLAGS:
    -h, --help                    Prints help information
    -i, --invert                  Invert background and foreground
    -n, --shift-sign-no-random    Don't randomize sign of col and row shift values
    -V, --version                 Prints version information

OPTIONS:
    -H, --hue <hue>                Sets initial hue of text color in degress [default: random]
    -S, --seed <seed>              Sets seed [default: random]
    -C, --shift-col <shift-col>    How many degrees to shift text color hue for every column [default: 1.6]
    -R, --shift-row <shift-row>    How many degrees to shift text color hue for every row [default: 3.2]
    -s, --style <style>            Rainbow mode [default: rainbow] [possible values: rainbow, sinebow]
```
## Screenshot
![a demo screenshot of lcat in action](.github/screenshot.png)
