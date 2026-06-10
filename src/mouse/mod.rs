#[cfg(target_os = "linux")]
mod hyprland;
#[cfg(target_os = "linux")]
use hyprland::HyprMouse;

#[cfg(not(target_os = "linux"))]
mod other;
#[cfg(not(target_os = "linux"))]
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

#[cfg(target_os = "linux")]
pub fn get_mouse() -> StdResult<Box<dyn Mouse>, Error> {
    match HyprMouse::new() {
        Ok(cursor) => Ok(Box::new(cursor)),
        Err(e) => Err(e),
    }
}

#[cfg(target_os = "linux")]
pub fn panic_if_mouse_pos_unsupported() {
    hyprland::panic_if_mouse_pos_unsupported();
}

#[cfg(not(target_os = "linux"))]
pub fn get_mouse() -> StdResult<Box<dyn Mouse>, Error> {
    match DefaultMouse::new() {
        Ok(mouse) => Ok(Box::new(mouse)),
        Err(e) => Err(e),
    }
}

#[cfg(not(target_os = "linux"))]
pub fn panic_if_mouse_pos_unsupported() {
    other::panic_if_mouse_pos_unsupported();
}

pub trait Mouse {
    fn get_position(&mut self) -> Result;
}
