// Adapted from rodio's signal generator implementation.
// Source: rodio v0.21.1, src/source/signal_generator.rs
// Upstream: https://github.com/RustAudio/rodio
// License: MIT OR Apache-2.0

use std::time::Duration;
use rodio::{ChannelCount, SampleRate, Source};
use rodio::source::{Function, GeneratorFunction};
use crate::signals::{sawtooth_signal, sine_signal, square_signal, triangle_signal};

pub struct MutableSignalGenerator {
    sample_rate: SampleRate,
    function: GeneratorFunction,
    frequency: f32,
    amplitude: f32,
    phase: f32,
}

impl MutableSignalGenerator {
    pub fn new(sample_rate: SampleRate, f: Function) -> Self {
        let function: GeneratorFunction = match f {
            Function::Sine => sine_signal,
            Function::Triangle => triangle_signal,
            Function::Square => square_signal,
            Function::Sawtooth => sawtooth_signal,
        };

        Self::with_function(sample_rate, function)
    }

    pub fn with_function(
        sample_rate: SampleRate,
        generator_function: GeneratorFunction,
    ) -> Self {
        MutableSignalGenerator {
            sample_rate,
            function: generator_function,
            frequency: 0.0,
            amplitude: 0.0,
            phase: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude
    }
}

impl Iterator for MutableSignalGenerator {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let period = self.sample_rate as f32 / self.frequency;
        let phase_step = 1.0f32 / period;

        let function = self.function;
        let val = self.amplitude * function(self.phase);
        self.phase = (self.phase + phase_step).rem_euclid(1.0f32);
        Some(val)
    }
}

impl Source for MutableSignalGenerator {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        1
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
