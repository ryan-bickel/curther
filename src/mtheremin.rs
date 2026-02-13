use std::sync::mpsc::{channel, Receiver};
use std::thread;
use rdev::{display_size, listen, Event, EventType};
use crate::theremin::Theremin;
use crate::Waveform;

pub struct MTheremin {
    theremin: Theremin,
    frequency: f32,
    amplitude: f32,
    width: u64,
    height: u64,
    rx: Receiver<Event>,
}

impl MTheremin {
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        let theremin = Theremin::new(frequency, amplitude, waveform);

        let (width, height) = display_size().unwrap();

        let (tx, rx) = channel();
        thread::spawn(move || {
            listen(move |event| {
                tx.send(event).unwrap();
            })
        });

        MTheremin {
            theremin,
            frequency,
            amplitude,
            width,
            height,
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
                    let amplitude = self.amplitude * (y as f32 / self.height as f32);
                    let frequency = self.frequency * (x as f32 / self.width as f32);

                    self.theremin.set_amplitude(amplitude);
                    self.theremin.set_frequency(frequency);
                }
                _ => {}
            }
        }
    }
}
