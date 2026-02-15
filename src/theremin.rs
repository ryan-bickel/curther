use std::fmt;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use atomic_float::AtomicF32;
use rodio::{OutputStream, OutputStreamBuilder, SampleRate, Source};
use rodio::source::Function;
use crate::mutable_signal_generator::MutableSignalGenerator;
use crate::waveform::Waveform;

pub struct Theremin {
    frequency: Arc<AtomicF32>,
    amplitude: Arc<AtomicF32>,
    _output_stream: OutputStream,
}

impl Theremin {
    fn new(frequency: Arc<AtomicF32>, amplitude: Arc<AtomicF32>, output_stream: OutputStream) -> Self {
        Theremin {
            frequency,
            amplitude,
            _output_stream: output_stream,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency.store(frequency, Ordering::Relaxed)
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude.store(amplitude, Ordering::Relaxed)
    }
}


pub struct ThereminBuilder {
    frequency: Arc<AtomicF32>,
    amplitude: Arc<AtomicF32>,
    refresh_rate: u32,
    sample_rate: SampleRate,
    output_stream: OutputStream,
    sources: Vec<Box<dyn Source + Send>>,
}

impl ThereminBuilder {
    pub fn new() -> Result<Self, ThereminBuildError> {
        let mut output_stream = OutputStreamBuilder::open_default_stream()
            .map_err(|_| ThereminBuildError::StreamCreation)?;
        output_stream.log_on_drop(false);
        let sample_rate = output_stream.config().sample_rate();

        Ok(ThereminBuilder {
            frequency: Arc::new(AtomicF32::new(0.0)),
            amplitude: Arc::new(AtomicF32::new(0.0)),
            refresh_rate: 1000,
            sample_rate,
            output_stream,
            sources: Vec::new(),
        })
    }

    pub fn refresh_rate(mut self, refresh_rate: u32) -> ThereminBuilder {
        self.refresh_rate = refresh_rate;
        self
    }

    pub fn add_voice(mut self, waveform: Waveform, interval: f32) -> Result<ThereminBuilder, ThereminBuildError> {
        if interval <= 0.0 {
            return Err(ThereminBuildError::InvalidInterval);
        }

        let frequency_clone = Arc::clone(&self.frequency);
        let amplitude_clone = Arc::clone(&self.amplitude);

        let source = MutableSignalGenerator::new(self.sample_rate, Function::from(waveform))
            .periodic_access(Duration::from_secs(1) / self.refresh_rate, move |src| {
                src.set_frequency(frequency_clone.load(Ordering::Relaxed) * interval);
                src.set_amplitude(amplitude_clone.load(Ordering::Relaxed));
            });

        self.sources.push(Box::new(source));
        Ok(self)
    }

    pub fn build(self) -> Result<Theremin, ThereminBuildError> {
        if self.sources.is_empty() {
            return Err(ThereminBuildError::NoVoices);
        }

        self.sources.into_iter().for_each(|source| {
            self.output_stream.mixer().add(source);
        });

        Ok(Theremin::new(
            self.frequency,
            self.amplitude,
            self.output_stream
        ))
    }
}

pub enum ThereminBuildError {
    InvalidInterval,
    NoVoices,
    StreamCreation,
}

impl fmt::Debug for ThereminBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ThereminBuildError::InvalidInterval => "interval must be greater than zero",
            ThereminBuildError::NoVoices => "no voices supplied",
            ThereminBuildError::StreamCreation => "unable to create output stream",
        };
        write!(f, "{}", msg)
    }
}
