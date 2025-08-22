mod game;
mod game_objects;

use asciibird::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::view::cursor;
use hewn::game_object::Coordinate;
#[cfg(not(target_arch = "wasm32"))]
use hewn::{
    view::{View, TerminalRenderer},
    runtime::{initialize_terminal_io, TerminalRuntime},
};

fn main() {
    play_asciibird_in_terminal();
}

#[cfg(not(target_arch = "wasm32"))]
fn play_asciibird_in_terminal() {
    use crate::game::default_game;

    let (stdout, stdin) = initialize_terminal_io();

    let mut game = default_game();
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = View {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(cursor::FollowPlayerXCursorStrategy::new()),
    };
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut display);

    runtime.start();
}
