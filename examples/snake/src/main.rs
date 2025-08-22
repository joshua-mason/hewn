mod game;
use crate::game::default_game;
use hewn::game_object::Coordinate;
#[cfg(not(target_arch = "wasm32"))]
use hewn::{
    display::BaseDisplay, initialize_terminal_io, runtime::TerminalRuntime, TerminalRenderer,
};

const SCREEN_WIDTH: u16 = 30;
const SCREEN_HEIGHT: u16 = 25;

fn main() {
    let (stdout, stdin) = initialize_terminal_io();
    let mut game = default_game();
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = BaseDisplay {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(hewn::display::cursor::StaticCursorStrategy::new()),
    };
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut display);

    runtime.start();
}
