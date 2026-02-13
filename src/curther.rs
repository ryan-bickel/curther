use std::sync::mpsc::{channel, Receiver};
use std::thread;
use rdev::{display_size, listen, Event, EventType, Key};
use crate::theremin::Theremin;
use crate::Waveform;

pub struct Curther {
    theremin: Theremin,
    frequency: f32,
    amplitude: f32,
    width: f64,
    height: f64,
    rx: Receiver<Event>,
}

impl Curther {
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        let theremin = Theremin::new(frequency, amplitude, waveform);

        let (width, height) = display_size()
            .expect("failed to get display dimensions");

        let (tx, rx) = channel();
        thread::spawn(move || {
            listen(move |event| {
                let _ = tx.send(event);
            })
        });

        Curther {
            theremin,
            frequency,
            amplitude,
            width: width as f64,
            height: height as f64,
            rx
        }
    }

    pub fn join(&mut self) {
        self.event_loop();
    }

    fn event_loop(&mut self) {
        for event in self.rx.iter() {
            match event.event_type {
                EventType::MouseMove {x, y} => {
                    let x = x.min(self.width).max(1.0);
                    let y = y.min(self.height).max(1.0);

                    let amplitude_multiplier = (self.height - y) / self.height;
                    let frequency_multiplier = x / self.width;

                    let amplitude = self.amplitude * amplitude_multiplier as f32;
                    let frequency = self.frequency * frequency_multiplier as f32;

                    self.theremin.set_amplitude(amplitude);
                    self.theremin.set_frequency(frequency);
                },
                EventType::KeyPress(Key::Escape) => {
                    return;
                },
                _ => {}
            }
        }
    }
}
