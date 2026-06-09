use std::{env, process::Stdio, str::FromStr};

use thiserror::Error;
use derive_more::Display;
use std::process::Command;
use mouse_position::mouse_position::{Mouse, Position};

const HYPRLAND_SIGNATURE: &'static str = "HYPRLAND_INSTANCE_SIGNATURE";

#[derive(Debug, Display, Error)]
pub enum MouseError {
    Unavailable,
}

type StdResult<T, E> = std::result::Result<T, E>;
pub type Result = StdResult<Position, MouseError>;

enum MouseKind {
    Hyprland,
    Other,
}

pub struct Cursor {
    mouse: MouseKind,
}

impl Cursor {
    pub fn new() -> Self {
        dbg!(env::var_os(HYPRLAND_SIGNATURE));
        let mouse = if env::var_os(HYPRLAND_SIGNATURE).is_some() {
            MouseKind::Hyprland
        } else {
            MouseKind::Other
        };

        Self {
            mouse
        }
    }

    pub fn get_position(&self) -> Result {
        match self.mouse {
            MouseKind::Hyprland => HyprMouse::get_mouse_position(),
            _ => {
                match Mouse::get_mouse_position() {
                    Mouse::Position {x, y} => Ok(Position { x, y }),
                    Mouse::Error => Err(MouseError::Unavailable),
                }
            },
        }
    }
}

struct HyprMouse;

impl HyprMouse {
    pub fn get_mouse_position() -> Result {
        let bytes = Command::new("hyprctl")
            .arg("cursorpos")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap()
            .stdout;
        let output = String::from_utf8(bytes).unwrap();

        let (x, y) = output.trim().split_once(", ").unwrap();
        let x = parse_pos(x)?;
        let y = parse_pos(y)?;

        Ok(Position { x, y })
    }
}

fn parse_pos(pos_str: &str) -> StdResult<i32, MouseError> {
    match str::parse(pos_str) {
        Ok(pos) => Ok(pos),
        Err(_) => Err(MouseError::Unavailable),
    }
}

