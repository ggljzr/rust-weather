# Rust-Weather

Super simple weather widget for [i3blocks](https://github.com/vivien/i3blocks). Possibly for other uses too, since it just prints current temperature and humidity to stdout.

## Installation

Build with

```
$ cargo build --release
```

## Usage

Binary: `target/release/rust-weather`. Requires API key from [openweathermap](https://openweathermap.org/) in `config.json`.

```
$ rust-weather /path/to/config.json
```

If argument is omitted, application looks for `config.json` in working directory.
