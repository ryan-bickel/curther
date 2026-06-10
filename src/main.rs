extern crate core;

mod curther;
mod theremin;
mod signals;
mod mutable_signal_generator;
mod waveform;
mod parser_utils;
mod mouse;

use std::thread;
use std::time::Duration;

use clap::{value_parser, Parser};
use crate::curther::{Curther, CurtherError};
<<<<<<< Updated upstream
use crate::waveform::Waveform;
use crate::parser_utils::parse_positive_f32;
use rodio::{OutputStreamBuilder, Source, source::SineWave};
=======
use crate::parser_utils::{get_devices, parse_device_name, parse_positive_f32};
use crate::waveform::Waveform;
use clap::{Parser, value_parser};
use rodio::{Device, DeviceTrait};
>>>>>>> Stashed changes

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

<<<<<<< Updated upstream
    #[arg(
        short = 't',
        long,
        default_value_t = false,
    )]
    test_tone: bool,
=======
    #[arg(short = 'l', long, default_value_t = false)]
    list_output_devices: bool,

    #[arg(
        short = 'o',
        long,
        value_parser = parse_device_name
    )]
    output_device: Option<Device>,
>>>>>>> Stashed changes
}

fn main() -> Result<(), CurtherError> {
    let Args {
        frequency,
        volume,
        waveform,
        intervals,
        polling_rate,
<<<<<<< Updated upstream
        test_tone,
=======
        list_output_devices,
        output_device,
>>>>>>> Stashed changes
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

    if list_output_devices {
        for device in get_devices().unwrap() {
            if let Ok(name) = device.name() {
                println!("{name}");
            }
        }
        return Ok(());
    }

    if let Some(device) = &output_device {
        let name = device.name().unwrap();
        println!("{name}");
    }

    let mut curther = Curther::new(
        frequency,
        volume,
        waveform,
        intervals,
        polling_rate,
        output_device,
    )?;
    curther.join();

    Ok(())
}
