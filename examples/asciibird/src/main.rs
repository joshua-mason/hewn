mod game;

use crate::game::create_game;
use hewn::view::cursor;
#[cfg(not(target_arch = "wasm32"))]
use hewn::{
    runtime::{initialize_terminal_io, TerminalRuntime},
    view::{TerminalRenderer, View},
};

pub const SCREEN_WIDTH: u16 = 50;
pub const SCREEN_HEIGHT: u16 = 30;

fn main() {
    play_asciibird_in_terminal();
}

#[cfg(not(target_arch = "wasm32"))]
fn play_asciibird_in_terminal() {
    let (stdout, stdin) = initialize_terminal_io();

    let mut game = create_game();
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = View {
        renderer: Box::new(renderer),
        view_cursor: hewn::view::ViewCoordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(cursor::FollowPlayerXCursorStrategy::new()),
    };
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut display);

    runtime.start();
}
