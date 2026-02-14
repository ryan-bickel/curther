mod curther;
mod theremin;
mod signals;
mod mutable_signal_generator;
mod waveform;

use clap::{value_parser, Parser};
use crate::curther::Curther;
use crate::waveform::Waveform;

#[derive(Parser)]
struct Args {
    /// waveform function
    #[arg(
        short = 'w',
        long,
        default_value_t = Waveform::Square
    )]
    waveform: Waveform,

    /// maximum frequency (20 - 20,000)
    #[arg(
        short = 'f',
        long,
        default_value_t = 1600,
        value_parser = value_parser!(u32).range(20..=20_000)
    )]
    frequency: u32,

    /// maximum volume (1 - 100)
    #[arg(
        short = 'v',
        long,
        default_value_t = 20,
        value_parser = value_parser!(u32).range(1..=100)
    )]
    volume: u32,

    #[arg(
        short = 'p',
        long,
        default_value_t = 1000,
        value_parser = value_parser!(u32).range(1..=1000)
    )]
    polling_rate: u32,
}

fn main() {
    let Args {
        frequency,
        volume,
        waveform,
        polling_rate
    } = Args::parse();

    let mut curther = Curther::new(frequency, volume, waveform, polling_rate);
    curther.join();
}
