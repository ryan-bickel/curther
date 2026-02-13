mod mtheremin;
mod theremin;
mod signals;
mod mutable_signal_generator;

use rodio::source::Function;
use crate::mtheremin::MTheremin;

fn main() {
    let mut mthere = MTheremin::new(880.0, 0.25, Function::Sawtooth);
    mthere.play();
}