mod mtheremin;
mod theremin;
mod signals;
mod mutable_signal_generator;
mod waveform;
mod parser_utils;

use clap::{Parser};
use crate::mtheremin::MTheremin;
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

    /// maximum frequency (at least 0)
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
}

fn main() {
    let Args { frequency, amplitude, waveform } = Args::parse();

    let mut mtheremin = MTheremin::new(frequency, amplitude, waveform);
    mtheremin.join();
}
