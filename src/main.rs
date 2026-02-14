mod curther;
mod theremin;
mod signals;
mod mutable_signal_generator;
mod waveform;
mod parser_utils;

use clap::{value_parser, Parser};
use crate::curther::Curther;
use crate::waveform::Waveform;
use crate::parser_utils::parse_f32_in_range;

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
        default_value_t = 1600.0,
        value_parser = parse_f32_in_range(20.0, 20_000.0)
    )]
    frequency: f32,

    /// maximum amplitude (0 - 1)
    #[arg(
        short = 'a',
        long,
        default_value_t = 0.2,
        value_parser = parse_f32_in_range(0.0, 1.0)
    )]
    amplitude: f32,

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
        amplitude,
        waveform,
        polling_rate
    } = Args::parse();

    let mut curther = Curther::new(frequency, amplitude, waveform, polling_rate);
    curther.join();
}
