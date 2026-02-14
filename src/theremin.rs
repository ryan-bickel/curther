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

impl Theremin {
    pub fn new(waveform: Waveform, refresh_rate: u32) -> Self {
        let mut output_stream = OutputStreamBuilder::open_default_stream()
            .expect("unable to create output stream");
        output_stream.log_on_drop(false);
        
        let sink = Sink::connect_new(&output_stream.mixer());

        let frequency_ref = Arc::new(AtomicF32::new(0.0));
        let frequency_ref_clone = Arc::clone(&frequency_ref);

        let amplitude_ref = Arc::new(AtomicF32::new(0.0));
        let amplitude_ref_clone = Arc::clone(&amplitude_ref);

        let source = MutableSignalGenerator::new(SAMPLE_RATE, Function::from(waveform))
            .periodic_access(Duration::from_secs(1) / refresh_rate, move |src| {
                src.set_frequency(frequency_ref_clone.load(Ordering::Relaxed));
                src.set_amplitude(amplitude_ref_clone.load(Ordering::Relaxed));
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
