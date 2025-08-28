//! Game logic trait and entity handling.

use crate::{ecs::ECS, runtime::Key};

// TODO rename to GameHandler to better conform to other naming conventions? e.g. winit app handler.
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
