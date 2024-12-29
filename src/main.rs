// TODO
// * Render platforms
// * Jump on hitting a platform
// * Move screen with the player
// * Points tally

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;

mod control;
mod display;
mod game;
mod io;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let mut display = display::Display::new(stdout);
    let mut control = control::Control::new(stdin, &mut game, &mut display);

    control.listen();
}
