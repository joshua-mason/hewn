use super::game_object::GameObject;

/// TODO: Add documentation for BaseGame trait
/// Core trait that all games must implement
pub trait BaseGame {
    /// TODO: Document start_game method
    fn start_game(&mut self);

    /// TODO: Document set_player_control_key method
    fn set_player_control_key(&mut self, key: Option<Key>);

    /// TODO: Document next method - advances game state
    fn next(&mut self);

    /// TODO: Document game_objects method - returns current game entities
    fn game_objects(&self) -> &[Box<dyn GameObject>];

    /// TODO: Document debug_str method - returns debug information
    fn debug_str(&self) -> Option<String>;
}

/// TODO: Add documentation for Entities struct
/// Container for managing game objects/entities
#[derive(Debug)]
pub struct Entities {
    pub game_objects: Vec<Box<dyn GameObject>>,
}

impl Entities {
    /// TODO: Document new method for Entities
    pub fn new() -> Entities {
        Entities {
            game_objects: vec![],
        }
    }

    /// TODO: Document add_game_objects method
    /// Adds game objects and sorts them by priority
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

/// TODO: Add documentation for Key enum
/// Represents all possible input keys for the game engine
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}

/// TODO: Document map_termion_key function
/// Maps termion key events to engine Key enum (non-WASM builds only)
#[cfg(not(target_arch = "wasm32"))]
pub fn map_termion_key(key: termion::event::Key) -> Key {
    match key {
        termion::event::Key::Left => Key::Left,
        termion::event::Key::Right => Key::Right,
        termion::event::Key::Up => Key::Up,
        termion::event::Key::Down => Key::Down,
        termion::event::Key::Char(' ') => Key::Space,
        termion::event::Key::Esc => Key::Escape,
        _ => todo!(),
    }
}
