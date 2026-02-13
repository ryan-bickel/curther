use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use rdev::{display_size, listen, EventType};
use rodio::source::Function;
use crate::theremin::Theremin;

pub struct MTheremin {
    theremin: Arc<Mutex<Theremin>>,
    frequency: f32,
    amplitude: f32,
}

impl MTheremin {
    pub fn new(frequency: f32, amplitude: f32, function: Function) -> Self {
        let theremin = Theremin::new(frequency, amplitude, function);
        let theremin_ref = Arc::new(Mutex::new(theremin));

        MTheremin {
            theremin: theremin_ref,
            frequency,
            amplitude
        }
    }

    pub fn play(&mut self) {
        let (width, height) = display_size().unwrap();

        let (tx, rx) = channel();
        thread::spawn(move || {
            listen(move |event| {
                tx.send(event).unwrap();
            })
        });

        for event in rx.iter() {
            match event.event_type {
                EventType::MouseMove {x, y} => {
                    let mut theremin = self.theremin.lock().unwrap();
                    let amplitude = self.amplitude * (y as f32 / height as f32);
                    let frequency = self.frequency * (x as f32 / width as f32);

                    theremin.set_amplitude(amplitude);
                    theremin.set_frequency(frequency);
                }
                _ => {}
            }
        }

        self.theremin.lock().unwrap().play();
    }
}
