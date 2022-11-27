# Audio Player

A very simple audio player. A slight abstraction over playing audio using the rust rodio library.

```
Usage: audio-player [--volume <volume>] [--speed <speed>] [--verbose <verbose>] input

Play an audio file and exit

Options:
  --volume          volume of the audio, 0.0 to 1.0. After 1.0 multiplies the
                    audio.
  --speed           speed of the audio default: 1.0
  --verbose         verbose default: false
  --help            display usage information
```

### Supports

- MP3
- WAV
- Vorbis
- FLAC

#### Soon:

- MP4
- ACC

## Motivation

A simple player that plays the audio file and quits. Needs to support major codecs.


## Development

It's a rust project, you know what to do...

```
cargo run -- --help
```

## Thanks

- rodio contributors
- cpal contributors
- argh contributors
