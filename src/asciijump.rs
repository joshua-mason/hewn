use crate::engine::{control::Control, initialize_terminal};
use game_objects::{platform::Platform, player_character::PlayerCharacter};

pub mod display;
pub mod game;
pub mod game_objects;

const WIDTH: usize = 10;
const HEIGHT: usize = 500;
const SCREEN_WIDTH: u16 = 10;
const SCREEN_HEIGHT: u16 = 20;

pub fn play_asciijump() {
    let (stdout, stdin) = initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_platforms(platforms);
    let mut display = display::Display::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut control = Control::new(stdin, &mut game, &mut display);

    control.listen();
}
