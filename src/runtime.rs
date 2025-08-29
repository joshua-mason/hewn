use crate::ecs::ECS;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Trait which all games must implement.
pub trait GameHandler {
    /// Start the game.
    fn start_game(&mut self);
    /// Compute the next game state based on player input.
    fn next(&mut self);
    /// Get the Entity Component System
    fn ecs(&self) -> &ECS;

    /// Get a string for debugging.
    fn debug_str(&self) -> Option<String>;

    fn handle_key(&mut self, key: Key, pressed: bool) -> bool;
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
}
