use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use atomic_float::AtomicF32;
use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};
use rodio::source::Function;
use crate::mutable_signal_generator::MutableSignalGenerator;
use crate::Waveform;

pub struct Theremin {
    frequency: Arc<AtomicF32>,
    amplitude: Arc<AtomicF32>,
    _output_stream: OutputStream,
    _sink: Sink,
}

impl Theremin {
    pub fn new(waveform: Waveform, refresh_rate: u32, harmonic_ratio: f32) -> Self {
        let mut output_stream = OutputStreamBuilder::open_default_stream()
            .expect("unable to create output stream");
        output_stream.log_on_drop(false);
        let sample_rate = output_stream.config().sample_rate();
        
        let sink = Sink::connect_new(&output_stream.mixer());

        let frequency= Arc::new(AtomicF32::new(0.0));
        let amplitude= Arc::new(AtomicF32::new(0.0));
        let n = AtomicBool::new(false);
        n.store(true);

        let make_source = |ratio: f32| {
            let frequency_clone = Arc::clone(&frequency);
            let amplitude_clone = Arc::clone(&amplitude);
            MutableSignalGenerator::new(sample_rate, Function::from(waveform))
                .periodic_access(Duration::from_secs(1) / refresh_rate, move |src| {
                    src.set_frequency(frequency_clone.load(Ordering::Relaxed) / ratio);
                    src.set_amplitude(amplitude_clone.load(Ordering::Relaxed));
                })
        };

        let f = Arc::new(1f32);
        *f = 5.0;

        if harmonic_ratio != 0.0 && harmonic_ratio != 1.0 {
            sink.append(make_source(1.0).mix(make_source(harmonic_ratio)));
        } else {
            sink.append(make_source(1.0));
        }

        Theremin {
            frequency,
            amplitude,
            _output_stream: output_stream,
            _sink: sink,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency.store(frequency, Ordering::Relaxed)
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude.store(amplitude, Ordering::Relaxed)
    }
}
