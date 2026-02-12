// mostly a copy of rodio's SignalGenerator with a mutable frequency

use std::time::Duration;
use rodio::{ChannelCount, SampleRate, Source};
use rodio::source::{Function, GeneratorFunction};
use crate::signals::{sawtooth_signal, sine_signal, square_signal, triangle_signal};

pub struct MutableSignalGenerator {
    sample_rate: SampleRate,
    function: GeneratorFunction,
    frequency: f32,
    phase: f32,
}

impl MutableSignalGenerator {
    pub fn new(sample_rate: SampleRate, frequency: f32, f: Function) -> Self {
        let function: GeneratorFunction = match f {
            Function::Sine => sine_signal,
            Function::Triangle => triangle_signal,
            Function::Square => square_signal,
            Function::Sawtooth => sawtooth_signal,
        };

        Self::with_function(sample_rate, frequency, function)
    }

    pub fn with_function(
        sample_rate: SampleRate,
        frequency: f32,
        generator_function: GeneratorFunction,
    ) -> Self {
        MutableSignalGenerator {
            sample_rate,
            function: generator_function,
            frequency,
            phase: 0.0,
        }
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

impl Iterator for MutableSignalGenerator {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let period = self.sample_rate as f32 / self.frequency;
        let phase_step = 1.0f32 / period;

        let f = self.function;
        let val = Some(f(self.phase));
        self.phase = (self.phase + phase_step).rem_euclid(1.0f32);
        val
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
