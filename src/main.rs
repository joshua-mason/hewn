// TODO
// * Render platforms
// * Jump on hitting a platform
// * Move screen with the player
// * Points tally

use game_object::platform::Platform;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;

mod control;
mod display;
mod game;
mod game_object;
mod io;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::from_tuples(&[(3, 3), (7, 9)]);
    game.set_platforms(platforms);
    let mut display = display::Display::new(stdout);
    let mut control = control::Control::new(stdin, &mut game, &mut display);

    control.listen();
}
