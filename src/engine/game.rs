//! Game logic trait and entity handling.

use crate::ecs::ECS;
use crate::runtime::Key;

/// Trait which all games must implement.
pub trait GameLogic {
    /// Start the game.
    fn start_game(&mut self);
    /// Compute the next game state based on player input.
    fn next(&mut self, key: Option<Key>);
    /// Get the Entity Component System
    fn ecs(&self) -> &ECS;

    /// Get a string for debugging.
    fn debug_str(&self) -> Option<String>;
}
