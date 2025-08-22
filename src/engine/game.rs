//! Game logic trait and entity handling.

use crate::game_object::GameObject;
use crate::runtime::Key;

/// Trait which all games must implement.
pub trait GameLogic {
    // Game logic
    /// Start the game.
    fn start_game(&mut self);
    /// Compute the next game state based on player input.
    fn next(&mut self, key: Option<Key>);

    // Game state
    /// Get the entities of the game.
    fn entities(&self) -> &Entities;

    // Debug rendering
    /// Get a string for debugging.
    fn debug_str(&self) -> Option<String>;
}

/// A collection of game objects.
#[derive(Debug)]
pub struct Entities {
    pub game_objects: Vec<Box<dyn GameObject>>,
}

impl Entities {
    /// Create a new collection of game objects.
    pub fn new() -> Entities {
        Entities {
            game_objects: vec![],
        }
    }

    /// Add a collection of game objects to the entities.
    pub fn add_game_objects(&mut self, game_objects: &mut Vec<Box<dyn GameObject>>) {
        self.game_objects.append(game_objects);

        self.game_objects.sort_by(|a, b| {
            if a.priority() == b.priority() {
                return std::cmp::Ordering::Equal;
            }
            if a.priority() > b.priority() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
    }
}
