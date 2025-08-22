use hewn::game::{Entities, GameLogic};
use hewn::game_object::utils::{
    collision_pass, maybe_get_concrete_type, maybe_get_concrete_type_mut, take_game_object,
};
use hewn::game_object::GameObject;
use hewn::runtime::Key;

use crate::game_objects::player_character::PlayerCharacter;
use crate::game_objects::wall::Wall;

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 30;
pub const SCREEN_WIDTH: u16 = 50;
pub const SCREEN_HEIGHT: u16 = 30;

pub fn default_game() -> Game {
    let mut game = Game::new();
    let walls = Wall::generate_walls(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_walls(walls);
    game
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InGame,
    Lost(usize),
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
    pub score: usize,
    pub entities: Entities,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            state: GameState::InGame,
            score: 0,
            entities: Entities::new(),
        };
        game.set_player(PlayerCharacter::new());
        game
    }

    fn move_player(&mut self, key: Option<Key>) {
        if let Some(Key::Up) = key {
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
        take_game_object::<PlayerCharacter>(&self.entities().game_objects)
    }

    pub fn get_mut_player_object(&mut self) -> Option<&mut PlayerCharacter> {
        self.entities
            .game_objects
            .iter_mut()
            .filter_map(|o| maybe_get_concrete_type_mut::<PlayerCharacter>(&mut **o))
            .next()
    }

    pub fn set_player(&mut self, player: PlayerCharacter) {
        let mut game_objects: Vec<Box<dyn GameObject>> = vec![Box::new(player)];
        if let Some(index) = self
            .entities
            .game_objects
            .iter()
            .position(|o| maybe_get_concrete_type::<PlayerCharacter>(&**o).is_some())
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

impl GameLogic for Game {
    fn start_game(&mut self) {
        self.score = 0;
        self.get_mut_player_object().unwrap().reset();
        self.state = GameState::InGame;
    }

    fn next(&mut self, key: Option<Key>) {
        if self.state != GameState::InGame {
            return;
        }

        self.move_player(key);
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

    fn entities(&self) -> &Entities {
        &self.entities
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
