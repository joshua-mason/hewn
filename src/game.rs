use std::any::Any;

use termion::event::Key;

use crate::asciijump::game_objects::platform::Platform;
use crate::asciijump::game_objects::player_character::{self, PlayerCharacter};
use crate::engine::game::BaseGame;
use crate::engine::game_object::utils::collision_pass;
use crate::engine::game_object::GameObject;

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

    pub game_objects: Vec<Box<dyn GameObject>>,

    pub state: GameState,
    pub score: usize,

    player_control_key: Option<Key>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
            state: GameState::Menu,
            score: 0,
            game_objects: vec![],
            player_control_key: None,
        }
    }

    pub fn add_game_objects(&mut self, game_objects: &mut Vec<Box<dyn GameObject>>) {
        self.game_objects.append(game_objects);
    }

    fn move_player(&mut self) {
        let width = self.width;
        match self.player_control_key {
            Some(Key::Left) => {
                if let Some(player) = self.get_mut_player_object() {
                    if player.coordinate.x > 0 {
                        player.move_left()
                    }
                }
            }
            Some(Key::Right) => {
                if let Some(player) = self.get_mut_player_object() {
                    if player.coordinate.x < width - 1 {
                        player.move_right()
                    }
                }
            }
            _ => {}
        }
    }
    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }

    pub fn get_player_object(&self) -> Option<&PlayerCharacter> {
        self.game_objects
            .iter()
            .filter_map(|o| {
                if let Some(player_character) = try_get_concrete_type::<PlayerCharacter>(&**o) {
                    Some(player_character)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn get_mut_player_object(&mut self) -> Option<&mut PlayerCharacter> {
        self.game_objects
            .iter_mut()
            .filter_map(|o| {
                if let Some(player_character) =
                    try_get_mut_concrete_type::<PlayerCharacter>(&mut **o)
                {
                    Some(player_character)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn get_platforms(&self) -> Vec<&Platform> {
        self.game_objects
            .iter()
            .filter_map(|o| {
                if let Some(platform) = try_get_concrete_type::<Platform>(&**o) {
                    Some(platform)
                } else {
                    None
                }
            })
            .collect::<Vec<&Platform>>()
    }

    fn set_platforms(&mut self, platforms: &mut Vec<Box<dyn GameObject>>) {
        // let game_objects = platforms
        //     .into_iter()
        //     .map(GameObject::Platform)
        //     .collect::<Vec<_>>();
        self.add_game_objects(platforms);
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
        self.game_objects.iter_mut().for_each(|o| o.next_step());

        collision_pass(&mut self.game_objects);

        // FIXME: can we improve the efficiency here? whole loop is not very good
        // FIXME: when two platforms, we don't definitely hit the closest one

        if self.get_player_object().unwrap().velocity < -6 {
            self.end_game();
        }

        self.score = self
            .score
            .max(self.get_player_object().unwrap().coordinate.y);
    }
}

fn try_get_concrete_type<T: Any>(abc: &dyn GameObject) -> Option<&T> {
    // 1. Convert &dyn Abc to &dyn Any using abc.as_any()
    // 2. Then downcast_ref::<T>()
    abc.as_any().downcast_ref::<T>()
}
fn try_get_mut_concrete_type<T: Any>(abc: &mut dyn GameObject) -> Option<&mut T> {
    // 1. Convert &dyn Abc to &dyn Any using abc.as_any()
    // 2. Then downcast_ref::<T>()
    abc.as_mut_any().downcast_mut()
}

// #[cfg(test)]
// mod test {
//     use crate::{
//         asciijump::game_objects::{
//             platform::Platform, player_character::PlayerCharacter, GameObject,
//         },
//         engine::game_object::Locate,
//     };

//     use super::Game;
//     #[test]
//     fn test_jump() {
//         let mut game = Game::new(10, 10);
//         game.start_game();
//         fast_forward(&mut game, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 6);

//         fast_forward(&mut game, 6);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 15);

//         fast_forward(&mut game, 4);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);
//     }

//     #[test]
//     fn test_start_on_platform() {
//         let mut game = Game::new(10, 10);
//         game.set_platforms(Platform::from_tuples(&[(1, 2)]));
//         game.start_game();
//         game.get_mut_player_object().unwrap().coordinate.y = 2;
//         game.get_mut_player_object().unwrap().coordinate.x = 2;
//         game.get_mut_player_object().unwrap().velocity = -5;

//         fast_forward(&mut game, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.x, 2);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 2);
//         assert_eq!(game.get_player_object().unwrap().velocity, 5);

//         fast_forward(&mut game, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 7);
//         assert_eq!(game.get_player_object().unwrap().velocity, 4);
//     }

//     #[test]
//     fn test_hit_platform() {
//         let mut game = Game::new(10, 10);
//         game.set_platforms(Platform::from_tuples(&[(1, 8)]));
//         game.add_game_objects(vec![
//             GameObject::PlayerCharacter(PlayerCharacter::new()),
//             GameObject::Platform(Platform::from_tuple((1, 8))),
//         ]);
//         game.start_game();

//         game.next();

//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 6);
//         fast_forward(&mut game, 9);
//         assert_eq!(game.get_player_object().unwrap().velocity, 5);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 8);

//         game.next();
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 13);
//     }

//     #[test]
//     fn test_player_game_object_hit_platform() {
//         let mut game = Game::new(10, 10);
//         game.set_platforms(Platform::from_tuples(&[(1, 8)]));
//         game.add_game_objects(vec![
//             GameObject::PlayerCharacter(PlayerCharacter::new()),
//             GameObject::Platform(Platform::from_tuple((1, 8))),
//         ]);
//         game.start_game();
//         game.next();

//         {
//             let player_object = game
//                 .game_objects
//                 .iter()
//                 .find(|o| match o {
//                     GameObject::PlayerCharacter(player_character) => true,
//                     GameObject::Platform(platform) => false,
//                 })
//                 .unwrap();

//             println!("asdas:{:?}", player_object);
//             assert_eq!(player_object.get_coords().y, 6);
//         }
//         fast_forward(&mut game, 1);
//         {
//             let player_object = game
//                 .game_objects
//                 .iter()
//                 .find(|o| match o {
//                     GameObject::PlayerCharacter(player_character) => true,
//                     GameObject::Platform(platform) => false,
//                 })
//                 .unwrap();

//             println!("asdas:{:?}", player_object);
//             assert_eq!(player_object.get_coords().y, 10);
//         }
//         fast_forward(&mut game, 7);
//         {
//             let player_object = game
//                 .game_objects
//                 .iter()
//                 .find(|o| match o {
//                     GameObject::PlayerCharacter(player_character) => true,
//                     GameObject::Platform(platform) => false,
//                 })
//                 .unwrap();

//             println!("asdas:{:?}", player_object);
//             assert_eq!(player_object.get_coords().y, 8);
//         }
//         println!("244");
//         fast_forward(&mut game, 1);

//         {
//             let player_object = game
//                 .game_objects
//                 .iter()
//                 .find(|o| match o {
//                     GameObject::PlayerCharacter(player_character) => true,
//                     GameObject::Platform(platform) => false,
//                 })
//                 .unwrap();

//             println!("asdas:{:?}", player_object);
//             assert_eq!(player_object.get_coords().y, 13);
//         }
//         // assert_eq!(game.get_player_object().unwrap().coordinate.y, 13);
//     }

//     #[test]
//     fn test_miss_platform() {
//         let mut game = Game::new(10, 10);
//         game.set_platforms(Platform::from_tuples(&[(3, 15)]));
//         game.start_game();

//         fast_forward(&mut game, 11);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);
//     }

//     #[test]
//     fn test_start_jump() {
//         let mut game = Game::new(10, 20);
//         let platforms = Platform::from_tuples(&[(1, 1)]);
//         game.set_platforms(platforms);
//         game.start_game();

//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);

//         game.next();
//         assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 6);

//         fast_forward(&mut game, 10);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);
//         assert_eq!(game.get_player_object().unwrap().velocity, 5);

//         fast_forward(&mut game, 5);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 16);
//     }

//     #[test]
//     fn test_fell_to_bottom_under_platform() {
//         let mut game = Game::new(10, 20);
//         let platforms = Platform::from_tuples(&[(1, 3)]);
//         game.set_platforms(platforms);
//         game.start_game();
//         game.get_mut_player_object().unwrap().velocity = 0;

//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);

//         game.next();
//         assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);

//         game.next();
//         assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 0);

//         game.next();
//         assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 0);
//         fast_forward(&mut game, 10);
//         assert_eq!(game.get_player_object().unwrap().coordinate.y, 0);
//     }

//     fn fast_forward(game: &mut Game, n: u16) {
//         for _ in 0..n {
//             game.next();
//         }
//     }
// }
