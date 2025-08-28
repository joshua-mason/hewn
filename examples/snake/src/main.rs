mod game;
use crate::game::create_game;
use hewn::game::GameHandler;
#[cfg(not(target_arch = "wasm32"))]
use hewn::runtime::{initialize_terminal_io, TerminalRuntime};
#[cfg(not(target_arch = "wasm32"))]
use hewn::view::{ScreenDimensions, TerminalRenderer, View};

const SCREEN_WIDTH: u16 = 100;
const SCREEN_HEIGHT: u16 = 100;

fn main() {
    let mut game = create_game(SCREEN_WIDTH, SCREEN_HEIGHT, None);
    game.start_game();
    hewn::render::app::run(Box::new(game)).unwrap();

    // let renderer = TerminalRenderer::new(
    //     stdout,
    //     ScreenDimensions {
    //         x: SCREEN_WIDTH,
    //         y: SCREEN_HEIGHT,
    //     },
    // );
    // let mut display = View {
    //     renderer: Box::new(renderer),
    //     view_cursor: hewn::view::ViewCoordinate { x: 0, y: 0 },
    //     cursor_strategy: Box::new(hewn::view::cursor::StaticCursorStrategy::new()),
    // };
    // let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut display);

    // runtime.start();
}
