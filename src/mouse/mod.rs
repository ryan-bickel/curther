#[cfg(unix)]
mod hyprland;
#[cfg(unix)]
use hyprland::HyprMouse;

#[cfg(not(unix))]
mod other;
#[cfg(not(unix))]
use other::DefaultMouse;

use derive_more::Display;
use mouse_position::mouse_position::Position;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum Error {
    Unavailable,
}

type StdResult<T, E> = std::result::Result<T, E>;
pub type Result = StdResult<Position, Error>;

#[cfg(unix)]
pub fn get_mouse() -> StdResult<Box<dyn Mouse>, Error> {
    match HyprMouse::new() {
        Ok(cursor) => Ok(Box::new(cursor)),
        Err(e) => Err(e),
    }
}

#[cfg(unix)]
pub fn panic_if_mouse_pos_unsupported() {
    hyprland::panic_if_mouse_pos_unsupported();
}

#[cfg(not(unix))]
pub fn get_mouse() -> StdResult<Box<dyn Mouse>, Error> {
    match DefaultMouse::new() {
        Ok(mouse) => Ok(Box::new(mouse)),
        Err(e) => Err(e),
    }
}

#[cfg(not(unix))]
pub fn panic_if_mouse_pos_unsupported() {
    other::panic_if_mouse_pos_unsupported();
}

pub trait Mouse {
    fn get_position(&mut self) -> Result;
}
