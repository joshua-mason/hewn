use super::game_object::GameObject;

pub trait BaseGame {
    fn start_game(&mut self);

    fn set_player_control_key(&mut self, key: Option<termion::event::Key>);

    fn next(&mut self);

    fn game_objects(&self) -> &[Box<dyn GameObject>];

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
            if (a.priority() == b.priority()) {
                return std::cmp::Ordering::Equal;
            }
            if (a.priority() > b.priority()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
    }
}
