use std::fmt;
use clap::ValueEnum;
use rodio::source::Function;

#[derive(ValueEnum, Copy, Clone)]
pub enum Waveform {
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

impl From<Waveform> for Function {
    fn from(value: Waveform) -> Self {
        match value {
            Waveform::Square => Function::Square,
            Waveform::Sawtooth => Function::Sawtooth,
            Waveform::Sine => Function::Sine,
            Waveform::Triangle => Function::Triangle,
        }
    }
}
