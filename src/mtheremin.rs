use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use rdev::{display_size, listen, Event, EventType};
use rodio::source::Function;
use crate::theremin::Theremin;

pub struct MTheremin {
    theremin: Arc<Mutex<Theremin>>,
    frequency: f32,
    amplitude: f32,
    width: u64,
    height: u64,
    rx: Receiver<Event>,
}

impl MTheremin {
    pub fn new(frequency: f32, amplitude: f32, function: Function) -> Self {
        let theremin = Theremin::new(frequency, amplitude, function);
        let theremin_ref = Arc::new(Mutex::new(theremin));

        let (width, height) = display_size().unwrap();

        let (tx, rx) = channel();
        thread::spawn(move || {
            listen(move |event| {
                tx.send(event).unwrap();
            })
        });

        MTheremin {
            theremin: theremin_ref,
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

    fn event_loop(&self) {
        for event in self.rx.iter() {
            match event.event_type {
                EventType::MouseMove {x, y} => {
                    let mut theremin = self.theremin.lock().unwrap();
                    let amplitude = self.amplitude * (y as f32 / self.height as f32);
                    let frequency = self.frequency * (x as f32 / self.width as f32);

                    theremin.set_amplitude(amplitude);
                    theremin.set_frequency(frequency);
                }
                _ => {}
            }
        }
    }
}
