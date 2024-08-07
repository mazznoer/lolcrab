# `lolcrab`

[![Build Status](https://github.com/mazznoer/lolcrab/actions/workflows/ci.yml/badge.svg)](https://github.com/mazznoer/lolcrab/actions)
[![crates.io](https://img.shields.io/crates/v/lolcrab.svg)](https://crates.io/crates/lolcrab)

Like [`lolcat`](https://github.com/busyloop/lolcat) but with [noise](https://en.wikipedia.org/wiki/OpenSimplex_noise) and more colorful. This is a fork of [lcat](https://github.com/davidkna/lcat-rs).

![lolcrab](docs/images/lolcrab.png)

## Installation

Pre-compiled binaries for Linux, macOS and Windows is avaliable on [release page](https://github.com/mazznoer/lolcrab/releases).

### Cargo

`lolcrab` can be installed using [cargo](https://www.rust-lang.org/tools/install).

```shell
cargo install lolcrab
```

## Usage

```text
Usage: lolcrab [OPTIONS] [File]...

Arguments:
  [File]...  [default: -]

Options:
  -g, --gradient <NAME>      Sets color gradient [default: rainbow] [possible values: cividis, cool,
                             cubehelix, fruits, inferno, magma, plasma, rainbow, rd-yl-gn, sinebow,
                             spectral, turbo, viridis, warm]
  -c, --custom <COLOR>...    Create custom gradient using the specified colors
      --sharp <NUM>          Sharp gradient
  -s, --scale <FLOAT>        Sets noise scale. Try value between 0.01 .. 0.2 [default: 0.034]
  -S, --seed <NUM>           Sets seed [default: random]
  -i, --invert               Colorize the background
  -r, --random-colors <NUM>  Use random colors as custom gradient [1 .. 100]
  -a, --animate              Enable animation mode
  -d, --duration <NUM>       Animation duration
      --speed <SPEED>        Animation speed
  -h, --help                 Print help information
  -V, --version              Print version information
```
