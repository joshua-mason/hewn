use asciijump::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::view::{cursor, ScreenDimensions};
#[cfg(not(target_arch = "wasm32"))]
use hewn::{
    runtime::{modname::initialize_terminal_io, modname::TerminalRuntime},
    view::{TerminalRenderer, View},
};

use crate::game::create_game;

pub mod game;

fn main() {
    play_asciijump_in_terminal();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_asciijump_in_terminal() {
    let (stdout, stdin) = modname::initialize_terminal_io();
    let mut game = create_game(None);
    // TODO where we input height and width as args, can we make it a struct so labelled instead of just
    // guessing?
    let renderer = TerminalRenderer::new(
        stdout,
        ScreenDimensions {
            x: SCREEN_WIDTH,
            y: SCREEN_HEIGHT,
        },
    );
    let mut display = View {
        renderer: Box::new(renderer),
        view_cursor: hewn::view::ViewCoordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(cursor::FollowPlayerYCursorStrategy::new()),
    };
    let mut runtime = modname::TerminalRuntime::new(stdin, &mut game, &mut display);

    runtime.start();
}
