use super::game_object::GameObject;

pub trait BaseGame {
    // Game logic
    fn start_game(&mut self);
    fn next(&mut self);

    // Game control
    fn set_player_control_key(&mut self, key: Option<Key>);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}

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
