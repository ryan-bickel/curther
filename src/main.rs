mod mthere;
mod theremin;
mod signals;
mod mutable_signal_generator;

use rodio::source::Function;
use crate::mthere::MThere;

fn main() {
    let mut mthere = MThere::new(880.0, 0.25, Function::Sawtooth);
    mthere.play();
}