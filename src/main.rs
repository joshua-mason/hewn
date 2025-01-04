use asciijump::game_objects::{platform::Platform, player_character::PlayerCharacter, GameObject};

const WIDTH: usize = 10;
const HEIGHT: usize = 500;
const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;
const SCREEN_HEIGHT: u16 = 20;

mod asciijump;
mod display;
mod engine;
mod game;
mod io;
mod utils;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    let mut game_objects = vec![GameObject::PlayerCharacter(PlayerCharacter::new())];
    let mut other = platforms
        .into_iter()
        .map(GameObject::Platform)
        .collect::<Vec<_>>();
    game_objects.append(&mut other);
    game.add_game_objects(game_objects);
    let mut display = display::Display::new(stdout, SCREEN_HEIGHT);
    let mut control = engine::control::Control::new(stdin, &mut game, &mut display);

    control.listen();
}
