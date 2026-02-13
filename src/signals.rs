// Adapted from rodio's signal generator implementation.
// Source: rodio v0.21.1, src/source/signal_generator.rs
// Upstream: https://github.com/RustAudio/rodio
// License: MIT OR Apache-2.0

use std::f32::consts::TAU;

pub fn sine_signal(phase: f32) -> f32 {
    (TAU * phase).sin()
}

pub fn triangle_signal(phase: f32) -> f32 {
    4.0f32 * (phase - (phase + 0.5f32).floor()).abs() - 1f32
}

pub fn square_signal(phase: f32) -> f32 {
    if phase % 1.0f32 < 0.5f32 {
        1.0f32
    } else {
        -1.0f32
    }
}

pub fn sawtooth_signal(phase: f32) -> f32 {
    2.0f32 * (phase - (phase + 0.5f32).floor())
}
