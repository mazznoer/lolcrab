# `lolcrab`

[![crates.io](https://img.shields.io/crates/v/lolcrab.svg)](https://crates.io/crates/lolcrab)

Like [`lolcat`](https://github.com/busyloop/lolcat) but with [noise](https://en.wikipedia.org/wiki/OpenSimplex_noise) and more colorful.

![lolcrab](docs/images/lolcrab.png)

## Installation

`lolcrab` can be installed with [cargo](https://www.rust-lang.org/tools/install).

```shell
cargo install lolcrab
```

## Usage

```text
USAGE:
    lolcrab [FLAGS] [OPTIONS] [--] [File]...

ARGS:
    <File>...    [default: -]

FLAGS:
    -h, --help       Print help information
    -i, --invert     Colorize the background
    -V, --version    Print version information

OPTIONS:
    -c, --custom <COLOR>...    Create custom gradient using the specified colors
    -g, --gradient <NAME>      Sets color gradient [default: rainbow] [possible values: cividis,
                               cool, cubehelix, inferno, magma, plasma, rainbow, rd-yl-gn, sinebow,
                               spectral, turbo, viridis, warm]
    -s, --scale <FLOAT>        Sets noise scale. Try value between 0.01 .. 0.2 [default: 0.034]
    -S, --seed <NUM>           Sets seed [default: random]
        --sharp <NUM>          Sharp gradient
```
