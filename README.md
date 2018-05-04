# Rust-Weather

Super simple weather widget for [i3blocks](https://github.com/vivien/i3blocks). Possibly for other uses too, since it just prints current temperature and humidity to stdout. WORK IN PROGRESS.

## Installation

Build with

```
$ cargo build --release
```

## Usage

Binary: `target/release/rust-weather`. Requires API key from [openweathermap](https://openweathermap.org/) in `config.json`. Since path to cofig file is hardcoded (for now), this file needs to be in the same place as binary.

## todos

Better config reading.