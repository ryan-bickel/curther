use std::sync::Arc;
use std::sync::atomic::Ordering;
use atomic_float::AtomicF32;
use rodio::OutputStream;

pub struct Theremin {
    frequency: Arc<AtomicF32>,
    amplitude: Arc<AtomicF32>,
    _output_stream: OutputStream,
}

impl Theremin {
    pub fn new(frequency: Arc<AtomicF32>, amplitude: Arc<AtomicF32>, output_stream: OutputStream) -> Self {
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
