use anyhow::{bail, Result};
use argh::FromArgs;
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

#[derive(FromArgs)]
/// Play an audio file and exit
struct Options {
    /// volume of the audio, 0.0 to 1.0. After 1.0 multiplies the audio.
    #[argh(option, short = 'v')]
    volume: Option<f32>,

    /// speed of the audio
    /// default: 1.0
    #[argh(option, short = 's')]
    speed: Option<f32>,

    /// verbose
    /// default: false
    #[argh(option, short = '!')]
    verbose: Option<bool>,

    /// input file
    #[argh(positional, greedy)]
    input: String,
}

fn log(opts: &Options) -> impl Fn(&str) -> () + '_ {
    let verbose = opts.verbose.clone();
    move |log: &str| {
        if let Some(_) = verbose {
            println!("{log}");
        }
    }
}

fn main() -> Result<()> {
    let opts: Options = argh::from_env();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(v) => v,
        Err(err) => {
            bail!(
                "Could not find the default physical sound device to play audio on. 
Error Message: {err}
Maybe we don't support your OS, but we couldn't find a default sound device."
            );
        }
    };

    let sink = Sink::try_new(&stream_handle).unwrap();

    if let Some(volume) = opts.volume {
        log(&opts)(&format!("volume: {volume}"));
        sink.set_volume(volume);
    }

    if let Some(speed) = opts.speed {
        log(&opts)(&format!("speed: {speed}"));
        sink.set_speed(speed);
    }

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(&opts.input).unwrap());

    log(&opts)(&format!("read: {:?}", &opts.input));
    // Decode that sound file into a source
    let source = match Decoder::new(file) {
        Ok(v) => v,
        Err(err) => {
            bail!("Could not decode this file: {:?}. 
Error Message: {err}. 
Consider re-encoding this audio file into one of the supported types. https://github.com/rockerBOO/audio-player#supports", &opts.input);
        }
    };

    sink.append(source);
    log(&opts)("playing...");

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
    log(&opts)("complete!");

    Ok(())
}
