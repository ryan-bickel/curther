use core::fmt;
use std::thread;
use std::time::Duration;
use mouse_position::mouse_position::{Mouse, Position};
use rdev::{display_size, listen, EventType, Key};
use crossbeam_channel::{bounded, select_biased, Receiver, RecvError};
use log::debug;
use crate::theremin::{Theremin, ThereminBuildError, ThereminBuilder};
use crate::Waveform;

pub struct Curther {
    theremin: Theremin,
    frequency: u32,
    volume: u32,
    width: u64,
    height: u64,
    rx_key: Receiver<Key>,
    rx_mouse: Receiver<Position>
}

impl Curther {
    pub fn new(
        frequency: u32,
        volume: u32,
        waveform: Waveform,
        intervals: Option<Vec<f32>>,
        polling_rate: u32
    ) -> Result<Self, CurtherError> {
        let mut builder = ThereminBuilder::new()?
            .refresh_rate(polling_rate)?
            .add_voice(waveform, 1.0)?;

        if let Some(intervals) = intervals {
            for interval in intervals {
                builder = builder.add_voice(waveform, interval)?
            }
        }

        let theremin = builder.build()?;

        let (width, height) = display_size()
            .expect("failed to get display dimensions");

        let rx_key = create_key_listener();
        let rx_mouse = create_mouse_poller(polling_rate);

        Ok(Curther {
            theremin,
            frequency,
            volume,
            width,
            height,
            rx_key,
            rx_mouse,
        })
    }

    pub fn join(&mut self) {
        self.event_loop();
    }

    fn event_loop(&mut self) {
        loop {
            select_biased! {
                recv(self.rx_mouse) -> msg => match msg {
                    Ok(Position {x, y}) => self.handle_mouse_moved(x, y),
                    Err(RecvError) => {
                        debug!("exiting due to mouse receive error");
                        return;
                    },
                },
                recv(self.rx_key) -> msg => match msg {
                    Ok(Key::Escape) => {
                        debug!("exiting normally");
                        return;
                    },
                    Ok(_) => {}
                    Err(RecvError) => {
                        debug!("exiting due to key receive error");
                        return;
                    },
                }
            }
        }
    }

    fn handle_mouse_moved(&mut self, x: i32, y: i32) {
        let x = x.min(self.width as i32).max(1) as u64;
        let y = y.min(self.height as i32).max(1) as u64;

        let amplitude_multiplier = (self.height - y) as f32 / self.height as f32;
        let frequency_multiplier = x as f32 / self.width as f32;

        let amplitude = self.volume as f32 / 100.0 * amplitude_multiplier;
        let frequency = self.frequency as f32 * frequency_multiplier;

        self.theremin.set_amplitude(amplitude);
        self.theremin.set_frequency(frequency);
    }
}

fn create_key_listener() -> Receiver<Key> {
    let (tx, rx) = bounded(0);
    thread::spawn(move || {
        listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                if let Err(e) = tx.send(key) {
                    debug!("key listening thread exiting due to send error: {}", e);
                    return;
                }
            }
        })
    });

    rx
}

fn create_mouse_poller(polling_rate: u32) -> Receiver<Position> {
    let (tx, rx) = bounded(1);
    thread::spawn(move || {
        let mut prev_x = i32::MIN;
        let mut prev_y = i32::MIN;

        loop {
            match Mouse::get_mouse_position() {
                Mouse::Position {x, y} => {
                    if x != prev_x || y != prev_y {
                        let _ = tx.try_send(Position {x, y});
                        prev_x = x;
                        prev_y = y;
                    }
                }
                Mouse::Error => {
                    debug!("unable to get mouse position")
                }
            };

            thread::sleep(Duration::from_secs(1) / polling_rate);
        }
    });

    rx
}

pub enum CurtherError {
    ThereminBuildError(ThereminBuildError)
}

impl From<ThereminBuildError> for CurtherError {
    fn from(err: ThereminBuildError) -> Self {
        CurtherError::ThereminBuildError(err)
    }
}

impl fmt::Debug for CurtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            CurtherError::ThereminBuildError(msg) => msg
        };
        write!(f, "failed to create theremin: {:?}", msg)
    }
}
