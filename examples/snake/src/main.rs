mod game;
use hewn::game_object::Coordinate;

#[cfg(not(target_arch = "wasm32"))]
use hewn::{control::TerminalControl, display::BaseDisplay, initialize_terminal, TerminalRenderer};
fn main() {
    play_snake_in_terminal();
}

const SCREEN_WIDTH: u16 = 30;
const SCREEN_HEIGHT: u16 = 25;

#[cfg(not(target_arch = "wasm32"))]
fn play_snake_in_terminal() {
    use crate::game::default;

    let (stdout, stdin) = initialize_terminal();
    let mut game = default();
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = BaseDisplay {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(hewn::display::cursor::StaticCursorStrategy::new()),
    };
    let mut control = TerminalControl::new(stdin, &mut game, &mut display);

    control.listen();
}
