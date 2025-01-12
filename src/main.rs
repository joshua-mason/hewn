use asciijump::{
    display,
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
mod game;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    let mut game_objects: Vec<Box<dyn GameObject>> = vec![Box::new(PlayerCharacter::new())];
    game.add_game_objects(&mut game_objects);
    let mut other: Vec<Box<dyn GameObject>> = platforms
        .into_iter()
        .map(|p| Box::new(p) as Box<dyn GameObject>)
        .collect::<Vec<Box<dyn GameObject>>>();
    game.add_game_objects(&mut other);
    let mut display = display::Display::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH as u16);
    let mut control = engine::control::Control::new(stdin, &mut game, &mut display);

    control.listen();
}
