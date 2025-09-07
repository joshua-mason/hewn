use std::time::Duration;

use crate::ecs::ECS;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Trait which all games must implement.
pub trait GameHandler {
    /// Start the game.
    fn start_game(&mut self);
    /// Compute the next game state based on player input.
    fn next(&mut self, dt: Duration);
    /// Get the Entity Component System
    fn ecs(&self) -> &ECS;

    /// Get a string for debugging.
    fn debug_str(&self) -> Option<String>;

    /// Handle a key event.
    fn handle_event(&mut self, event: RuntimeEvent) -> bool;
}

pub struct KeyEvent {
    pub key: Key,
    pub pressed: bool,
}

pub enum RuntimeEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

/// Key for player control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
    Q,
}

/// Mouse location in world co-ordinates
#[derive(Clone, Copy, Debug)]
pub struct MouseLocation {
    pub x: f32,
    pub y: f32,
}

/// Key for player control.
#[derive(Clone, Copy, Debug)]
pub enum MouseEvent {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
    CursorMoved(MouseLocation),
}
