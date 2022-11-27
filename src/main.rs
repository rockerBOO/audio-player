use argh::FromArgs;
use rodio::{Decoder, OutputStream};
use rodio::{PlayError, Sink};
use std::fs::File;
use std::io::BufReader;

#[derive(FromArgs)]
/// Play an audio file and exit
struct Options {
    /// volumne of the audio, 0.0 to 1.0. After 1.0 multiplies the audio.
    #[argh(option)]
    volume: Option<f32>,

    /// speed of the audio
    /// default: 1.0
    #[argh(option)]
    speed: Option<f32>,

    /// verbose
    /// default: false
    #[argh(option)]
    verbose: Option<bool>,

    /// input file
    #[argh(positional, greedy)]
    input: String,
}

fn main() -> Result<(), PlayError> {
    let opts: Options = argh::from_env();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    if let Some(volume) = opts.volume {
        if let Some(verbose) = opts.verbose {
            if verbose {
                println!("volume: {volume}");
            }
        }
        sink.set_volume(volume);
    }

    if let Some(speed) = opts.speed {
        if let Some(verbose) = opts.verbose {
            if verbose {
                println!("speed: {speed}");
            }
        }
        sink.set_speed(speed);
    }

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(&opts.input).unwrap());

    if let Some(verbose) = opts.verbose {
        if verbose {
            println!("read: {:?}", &opts.input);
        }
    }
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    if let Some(verbose) = opts.verbose {
        if verbose {
            println!("playing...");
        }
    }

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();

    if let Some(verbose) = opts.verbose {
        if verbose {
            println!("complete!");
        }
    }

    Ok(())
}
