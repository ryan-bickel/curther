mod mtheremin;
mod theremin;
mod signals;
mod mutable_signal_generator;

use std::env;
use rodio::source::Function;
use crate::mtheremin::MTheremin;

fn main() {
    let args: Vec<String> = env::args().collect();

    let function = if args.len() < 3 {
        Function::Square
    } else {
        let function_string = args[2].as_str();
        match function_string {
            "square" => Function::Square,
            "sawtooth" => Function::Sawtooth,
            "sine" => Function::Sine,
            "triangle" => Function::Triangle,
            _ => Function::Square
        }
    };

    let mut mtheremin = MTheremin::new(2400.0, 0.25, function);
    mtheremin.play();
}