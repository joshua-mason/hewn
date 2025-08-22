//! Game logic trait and entity handling.

use crate::game_object::GameObject;
use crate::runtime::Key;

pub trait GameLogic {
    // Game logic
    fn start_game(&mut self);
    fn next(&mut self, key: Option<Key>);

    // Game state
    fn entities(&self) -> &Entities;

    // render
    fn debug_str(&self) -> Option<String>;
}

#[derive(Debug)]
pub struct Entities {
    pub game_objects: Vec<Box<dyn GameObject>>,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            game_objects: vec![],
        }
    }

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
