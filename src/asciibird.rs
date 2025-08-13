use crate::engine::{control::Control, initialize_terminal};
use game_objects::{player_character::PlayerCharacter, wall::Wall};

pub mod display;
pub mod game;
pub mod game_objects;

const WIDTH: usize = 1000;
const HEIGHT: usize = 30;
const SCREEN_WIDTH: u16 = 50;
const SCREEN_HEIGHT: u16 = 30;

pub fn play_asciibird() {
    let (stdout, stdin) = initialize_terminal();
    let mut game = game::Game::new();
    let walls = Wall::generate_walls(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_walls(walls);
    let mut display = display::Display::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut control = Control::new(stdin, &mut game, &mut display);

    control.listen();
}
