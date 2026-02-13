mod mtheremin;
mod theremin;
mod signals;
mod mutable_signal_generator;

use std::fmt;
use clap::{Parser, ValueEnum};
use crate::mtheremin::MTheremin;

fn parse_f32(s: &str, min: f32, max: f32) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| "must be a floating point number")?;
    if (min..=max).contains(&v) {
        Ok(v)
    } else {
        Err(format!("must be between {} and {} (inclusive)", min, max))
    }
}

fn parse_amplitude(s: &str) -> Result<f32, String> {
    parse_f32(s, 0.0, 1.0)
}

fn parse_frequency(s: &str) -> Result<f32, String> {
    parse_f32(s, 20.0, 20_000.0)
}

#[derive(Parser)]
struct Args {
    /// waveform function
    #[arg(short = 'w', long, default_value_t = Waveform::Square)]
    waveform: Waveform,

    /// maximum frequency (at least 0)
    #[arg(short = 'r', long, default_value_t = 1600.0, value_parser = parse_frequency)]
    frequency: f32,

    /// maximum amplitude (0 - 1)
    #[arg(short = 'a', long, default_value_t = 0.2, value_parser = parse_amplitude)]
    amplitude: f32,
}

#[derive(ValueEnum, Copy, Clone)]
enum Waveform {
    Square,
    Sawtooth,
    Sine,
    Triangle,
}

impl fmt::Display for Waveform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Waveform::Square => "square".to_string(),
            Waveform::Sawtooth => "sawtooth".to_string(),
            Waveform::Sine => "sine".to_string(),
            Waveform::Triangle => "triangle".to_string(),
        };
        write!(f, "{}", str)
    }
}

fn main() {
    let Args { frequency, amplitude, waveform } = Args::parse();

    let mut mtheremin = MTheremin::new(frequency, amplitude, waveform);
    mtheremin.join();
}