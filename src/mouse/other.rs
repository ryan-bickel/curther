use crate::mouse::Result;
use crate::mouse::Error;
use crate::mouse::StdResult;
use crate::mouse::Mouse;
use mouse_position::{mouse_position, Position};

pub fn panic_if_mouse_pos_unsupported() {
    // assume supported
}

pub struct DefaultMouse {}

impl DefaultMouse {
    pub fn new() -> StdResult<Self, Error> {
        Ok(Self {})
    }
}

impl Mouse for DefaultMouse {
    fn get_position(&mut self) -> Result {
        match mouse_position::Mouse::get_mouse_position() {
            mouse_position::Mouse::Position { x, y } => Ok(Position { x, y }),
            mouse_position::Mouse::Error => Err(Error::Unavailable),
        }
    }
}
