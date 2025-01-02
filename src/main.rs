// TODO
// * Render platforms
// * Jump on hitting a platform
// * Move screen with the player
// * Points tally

use game_object::platform::Platform;

const WIDTH: usize = 10;
const HEIGHT: usize = 500;
const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;
const SCREEN_HEIGHT: u16 = 20;

mod control;
mod display;
mod game;
mod game_object;
mod io;
mod utils;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    game.set_platforms(platforms);
    let mut display = display::Display::new(stdout, SCREEN_HEIGHT);
    let mut control = control::Control::new(stdin, &mut game, &mut display);

    control.listen();
}
