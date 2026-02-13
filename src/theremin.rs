use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use atomic_float::AtomicF32;
use rodio::{OutputStream, OutputStreamBuilder, SampleRate, Sink, Source};
use rodio::source::{Function};
use crate::mutable_signal_generator::MutableSignalGenerator;
use crate::Waveform;

const SAMPLE_RATE: SampleRate = 48_000;

pub struct Theremin {
    frequency: Arc<AtomicF32>,
    amplitude: Arc<AtomicF32>,
    _sink: Sink,
    _output_stream: OutputStream,
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

impl Theremin {
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        let output_stream = OutputStreamBuilder::open_default_stream()
            .expect("unable to create output stream");
        let sink = Sink::connect_new(&output_stream.mixer());

        let frequency_ref = Arc::new(AtomicF32::new(frequency));
        let frequency_ref_clone = Arc::clone(&frequency_ref);

        let amplitude_ref = Arc::new(AtomicF32::new(amplitude));
        let amplitude_ref_clone = Arc::clone(&amplitude_ref);

        let source = MutableSignalGenerator::new(SAMPLE_RATE, frequency, amplitude, Function::from(waveform))
            .periodic_access(Duration::from_millis(1000 / 60), move |src| {
                src.set_frequency(frequency_ref_clone.load(Ordering::Relaxed));
                src.set_amplitude(amplitude_ref_clone.load(Ordering::Relaxed))
            });
        sink.append(source);


        Theremin {
            frequency: frequency_ref,
            amplitude: amplitude_ref,
            _sink: sink,
            _output_stream: output_stream
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency.store(frequency, Ordering::Relaxed)
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude.store(amplitude, Ordering::Relaxed)
    }
}
