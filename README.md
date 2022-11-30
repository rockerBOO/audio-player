# Audio Player

A very simple audio player. A slight abstraction over playing audio using the rust rodio library.

![audio player](https://user-images.githubusercontent.com/15027/204857859-384034b9-349c-471d-b36c-efa4b4b53292.png)

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

### Cargo

```
cargo install --git https://github.com/rockerBOO/audio-player.git audio-player
audio-player --help
```

### Manual

```
cargo build --release
./target/release/audio-player --help
```

## Audio conversion

We'll use some `ffmpeg` or `avconv` to do some conversion to make good samples for playing with commands.

Take a video file and extract only the first audio channel. Also works to convert audio files into ogg/vorbis.

```
ffmpeg -i lsd.ogg -acodec libvorbis -c copy -map 0:a -vn -sn output.ogg
```

`-t` used for duration, can apply for the input (before the `-i`) and to the output (after the `-i`).

```
ffmpeg -t 5 -i output.ogg -t 5 output-5seconds.ogg
```

5 seconds into the file, for 5 seconds on the output, so 00:05:00 to 00:10:00

`-to` gives a position, so here we can say the position in the audio file.

```
ffmpeg -to 00:05:00 -i output.ogg -to 00:10:00 output-5seconds.ogg
```

And a quick one-liner to have it play right away.

```
ffmpeg -to 00:05:00 -i output.ogg -to 00:10:00 output-5seconds.ogg && audio-player output-5seconds.ogg
```

## Samples

Download an audio file from some source

```
yt-dlp -f 'ba' -x --audio-format vorbis URL
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
