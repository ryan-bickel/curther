extern crate core;

mod curther;
mod mouse;
mod mutable_signal_generator;
mod parser_utils;
mod signals;
mod theremin;
mod waveform;

use std::thread;
use std::time::Duration;

use crate::curther::{Curther, CurtherError};
use crate::parser_utils::parse_positive_f32;
use crate::waveform::Waveform;
use clap::{Parser, value_parser};
use rodio::{OutputStreamBuilder, Source, source::SineWave};

#[derive(Parser)]
struct Args {
    /// waveform function
    #[arg(
        short = 'w',
        long,
        default_value_t = Waveform::Square
    )]
    waveform: Waveform,

    /// maximum frequency, hz (20 - 20,000)
    #[arg(
        short = 'f',
        long,
        default_value_t = 1600,
        value_parser = value_parser!(u32).range(20..=20_000)
    )]
    frequency: u32,

    /// maximum volume, % (1 - 100)
    #[arg(
        short = 'v',
        long,
        default_value_t = 20,
        value_parser = value_parser!(u32).range(1..=100)
    )]
    volume: u32,

    /// space-separated list of intervals ( > 0.0 ) [default: disabled]
    #[arg(
        short = 'i',
        long,
        num_args = 1..,
        value_parser = parse_positive_f32
    )]
    intervals: Option<Vec<f32>>,

    /// mouse polling rate, hz (1 - 1000)
    #[arg(
        short = 'p',
        long,
        default_value_t = 1000,
        value_parser = value_parser!(u32).range(1..=1000)
    )]
    polling_rate: u32,

    #[arg(short = 't', long, default_value_t = false)]
    test_tone: bool,
}

fn main() -> Result<(), CurtherError> {
    let Args {
        frequency,
        volume,
        waveform,
        intervals,
        polling_rate,
        test_tone,
    } = Args::parse();

    if test_tone {
        let output_stream = OutputStreamBuilder::open_default_stream()
            .expect("should be able to create output stream");
        output_stream
            .mixer()
            .add(SineWave::new(440.0).amplify(0.20));
        thread::sleep(Duration::from_secs(10));
        return Ok(());
    }

    let mut curther = Curther::new(frequency, volume, waveform, intervals, polling_rate)?;
    curther.join();

    Ok(())
}
