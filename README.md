# smhkd [![](https://img.shields.io/crates/v/smhkd.svg)](https://crates.io/crates/smhkd) [![License](https://img.shields.io/badge/license-ISC-blue.svg)](/LICENSE)

**smhkd** (Simple MIDI hotkontrol daemon) is a daemon that reacts to MIDI events by executing commands.

It's inspired by [sxhkd](https://github.com/baskerville/sxhkd), but handles ALSA MIDI events rather than X events.

# Current Status

smhkd is still in design phase and not yet ready for mainstream usage.

At the moment, *and at the moment only*, it reads a configuration file in `~/.config/smhkd/smhkdrc.json`, which looks like this:

```json
{
    "@nanoKONTROL2": {
        "0": "pactl set-sink-volume @DEFAULT_SINK@ $VALUE%",
        "+32": "pactl set-sink-volume @DEFAULT_SINK@ 100%",
        "+48": "pactl set-sink-volume @DEFAULT_SINK@ 30%",
        "+64": "pactl set-sink-volume @DEFAULT_SINK@ 0%",

        "+41": "playerctl play-pause",
        "+42": "playerctl stop",
        "+43": "playerctl position 15-",
        "+44": "playerctl position 15+",
        "+58": "playerctl previous",
        "+59": "playerctl next",

        "+60": "ddcutil --display 2 setvcp 60 0x11",
        "+62": "ddcutil --display 2 setvcp 60 0x0f",

        "7": "v4l2-ctl -d /dev/video0 --set-ctrl=zoom_absolute=$VALUE"
    },
    "129:0": {
        "1": "pactl set-sink-volume @DEFAULT_SINK@ $VALUE%",
        "+67": "pactl set-sink-volume @DEFAULT_SINK@ 100%",
        "+66": "pactl set-sink-volume @DEFAULT_SINK@ 30%",
        "+64": "pactl set-sink-volume @DEFAULT_SINK@ 0%"
    }
}
```

and it listens to each pair of client and port (`28:0` — where `28` is the client number for `nanoKONTROL2` and `0` the first available port for it — `129:0` and so on…) for MIDI events.

Every time it receives an event, it runs the command associated with the controller ID (if any). Occurences of `$VALUE` in that command are replaced with the value of the event. Buttons are usually associated with value 127 for press and value 0 for release, so to avoid buttons running commands twice, you can prefix the controller ID with either `+` (to run command on press) or `-` (to run command on release). In the example above, it:
 - sets the volume of a PulseAudio sink (which can actually be a PipeWire);
 - controls a music / video player;
 - switches the active input source of a DisplayPort / HDMI monitor;
 - sets the zoom level of a webcam
but any command could be executed instead.

Keep in mind that this configuration format is only temporary and will ultimately be replaced with something more similar to what *sxhkd* uses. There is no plan to provide backward compatibility with JSON or even a migration path when than happens.

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

## Running on startup

Run `systemctl --user enable --now smhkd` after installing.

## Compiling

Run `cargo build --release` in your working copy.

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/smhkd/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/smhkd/issues).

## License

smhkd is copyright (C) 2022-2024 Jérémie Roquet <jroquet@arkanosis.net> and licensed under the ISC license.

