use asciijump::{
    display, game,
    game_objects::{platform::Platform, player_character::PlayerCharacter},
};
use engine::{game_object::GameObject, io};

const WIDTH: usize = 10;
const HEIGHT: usize = 500;
const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;
const SCREEN_WIDTH: u16 = 10;
const SCREEN_HEIGHT: u16 = 20;

mod asciijump;
mod engine;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_platforms(platforms);
    let mut display = display::Display::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH as u16);
    let mut control = engine::control::Control::new(stdin, &mut game, &mut display);

    control.listen();
}
