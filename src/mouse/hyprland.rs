use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use mouse_position::mouse_position::{Position};
use crate::mouse::Result;
use crate::mouse::StdResult;
use crate::mouse::Error;
use crate::mouse::Mouse;

const HYPRLAND_SIGNATURE: &'static str = "HYPRLAND_INSTANCE_SIGNATURE";

pub fn panic_if_mouse_pos_unsupported() {
    if env::var_os(HYPRLAND_SIGNATURE).is_none() {
        panic!("Only hyprland is supported on Linux at this time.");
    }
}

pub struct HyprMouse {
    socket_path: String,
}

impl HyprMouse {
    pub fn new() -> StdResult<Self, Error> {
        let runtime = env::var("XDG_RUNTIME_DIR").map_err(|_| Error::Unavailable)?;
        let hyprland_signature = env::var(HYPRLAND_SIGNATURE).map_err(|_| Error::Unavailable)?;
        let socket_path = format!("{runtime}/hypr/{hyprland_signature}/.socket.sock");
        
        Ok(Self { socket_path })
    }
}

impl Mouse for HyprMouse {
    fn get_position(&mut self) -> Result {
        let mut stream = UnixStream::connect(&self.socket_path).map_err(|_| Error::Unavailable)?;
        stream.write_all(b"cursorpos").map_err(|_| Error::Unavailable)?;

        let mut output = String::new();
        stream.read_to_string(&mut output).map_err(|_| Error::Unavailable)?;

        let (x, y) = output.trim().split_once(", ").ok_or(Error::Unavailable)?;
        let x = parse_pos(x)?;
        let y = parse_pos(y)?;

        Ok(Position { x, y })
    }
}

fn parse_pos(pos_str: &str) -> StdResult<i32, Error> {
    match str::parse(pos_str) {
        Ok(pos) => Ok(pos),
        Err(_) => Err(Error::Unavailable),
    }
}

