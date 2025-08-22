mod game;
use crate::game::default_game;
use hewn::game_object::Coordinate;
#[cfg(not(target_arch = "wasm32"))]
use hewn::runtime::{initialize_terminal_io, TerminalRuntime};
#[cfg(not(target_arch = "wasm32"))]
use hewn::view::{TerminalRenderer, View};

const SCREEN_WIDTH: u16 = 40;
const SCREEN_HEIGHT: u16 = 10;

fn main() {
    let (stdout, stdin) = initialize_terminal_io();
    let mut game = default_game(SCREEN_WIDTH, SCREEN_HEIGHT);
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = View {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(hewn::view::cursor::StaticCursorStrategy::new()),
    };
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut display);

    runtime.start();
}
