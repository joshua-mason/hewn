use super::game_objects::player_character::PlayerCharacter;
use super::game_objects::wall::Wall;
use crate::engine::{
    collision_pass, game_object::utils::take_game_object, try_get_concrete_type,
    try_get_mut_concrete_type, BaseGame, Entities, GameObject,
};
use termion::event::Key;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InGame,
    Menu,
    Lost(usize),
}

#[derive(Debug)]
pub struct Game {
    pub width: usize,
    pub height: usize,

    pub state: GameState,
    pub score: usize,

    pub entities: Entities,
    player_control_key: Option<Key>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        let mut game = Game {
            width,
            height,
            state: GameState::Menu,
            score: 0,
            entities: Entities::new(),
            // game_objects: vec![],
            player_control_key: None,
        };
        game.set_player(PlayerCharacter::new());
        game
    }

    fn move_player(&mut self) {
        if let Some(Key::Up) = self.player_control_key {
            if let Some(player) = self.get_mut_player_object() {
                if player.coordinate.x > 0 {
                    player.jump()
                }
            }
        }
    }
    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }

    pub fn get_player_object(&self) -> Option<&PlayerCharacter> {
        take_game_object::<PlayerCharacter>(self.game_objects())
    }

    pub fn get_mut_player_object(&mut self) -> Option<&mut PlayerCharacter> {
        self.entities
            .game_objects
            .iter_mut()
            .filter_map(|o| try_get_mut_concrete_type::<PlayerCharacter>(&mut **o))
            .next()
    }

    pub fn set_player(&mut self, player: PlayerCharacter) {
        let mut game_objects: Vec<Box<dyn GameObject>> = vec![Box::new(player)];
        if let Some(index) = self
            .entities
            .game_objects
            .iter()
            .position(|o| try_get_concrete_type::<PlayerCharacter>(&**o).is_some())
        {
            self.entities.game_objects.remove(index);
        }
        self.entities.add_game_objects(&mut game_objects);
    }

    pub fn set_walls(&mut self, walls: Vec<Wall>) {
        let mut game_objects = walls
            .into_iter()
            .map(|p| Box::new(p) as Box<dyn GameObject>)
            .collect::<Vec<Box<dyn GameObject>>>();
        self.entities.add_game_objects(&mut game_objects);
    }
}

impl BaseGame for Game {
    fn set_player_control_key(&mut self, key: Option<termion::event::Key>) {
        self.player_control_key = key
    }

    fn start_game(&mut self) {
        self.score = 0;
        self.get_mut_player_object().unwrap().reset();
        self.state = GameState::InGame;
    }

    fn next(&mut self) {
        if self.state != GameState::InGame {
            return;
        }

        self.move_player();
        self.entities
            .game_objects
            .iter_mut()
            .for_each(|o| o.next_step());

        collision_pass(&mut self.entities.game_objects);

        // FIXME: can we improve the efficiency here? whole loop is not very good
        // FIXME: when two platforms, we don't definitely hit the closest one

        if self.get_player_object().unwrap().velocity < -10 {
            self.end_game();
        }
        if self.get_player_object().unwrap().hit_wall {
            self.end_game();
        }

        self.score = self
            .score
            .max(self.get_player_object().unwrap().coordinate.x);
    }

    fn game_objects(&self) -> &[Box<dyn GameObject>] {
        &self.entities.game_objects
    }

    fn debug_str(&self) -> Option<String> {
        if let Some(player) = self.get_player_object() {
            let a = format!(
                "v = {:4}, x = {:3}, y = {:3}",
                player.velocity, player.coordinate.x, player.coordinate.y
            );
            Some(a)
        } else {
            None
        }
    }
}
