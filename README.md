# smhkd [![](https://img.shields.io/crates/v/smhkd.svg)](https://crates.io/crates/smhkd) [![License](https://img.shields.io/badge/license-ISC-blue.svg)](/LICENSE)

**smhkd** (Simple MIDI hotkontrol daemon) is a daemon that reacts to MIDI events by executing commands.

It's inspired by [sxhkd](https://github.com/baskerville/sxhkd), but handles ALSA MIDI events rather than X events.

# Current Status

smhkd is still in design phase and not yet ready for mainstream usage.

## Usage

```
Usage: smhkd list
       smhkd run
       smhkd -h | --help
       smhkd --version

Commands:
    list                     List available MIDI controllers.
    run                      Listen to MIDI events and run commands.

Arguments:

Options:
    -h, --help               Show this screen.
    --version                Show version.
```

## Compiling

Run `cargo build --release` in your working copy.

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/smhkd/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/smhkd/issues).

## License

smhkd is copyright (C) 2022 Jérémie Roquet <jroquet@arkanosis.net> and licensed under the ISC license.

