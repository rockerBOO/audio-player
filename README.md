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

## Install

### Manual

Not packaged up yet so you'll need to clone this repo and build your own version.

```
cargo build --release
```

And you can alias it...

```
alias audio-player="/path/to/audio-player/target/release/audio-player"
```

Or make a link to the binary...

```
ln -sn /path/to/audio-player/target/release/audio-player ~/bin/audio-player
```

Or just move the binary where where you want.

## Audio conversion

Take a video file and extract only the first audio channel. Also works to convert audio files into ogg/vorbis.

```
ffmpeg -i lsd.ogg -acodec libvorbis -c copy -map 0:a -vn -sn output.ogg
```

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
