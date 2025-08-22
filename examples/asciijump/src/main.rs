use asciijump::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::cursor;
use hewn::{game_object::Coordinate, BaseDisplay};
#[cfg(not(target_arch = "wasm32"))]
use hewn::{initialize_terminal_io, runtime::TerminalRuntime, TerminalRenderer};

use crate::game::default_game;

pub mod game;
pub mod game_objects;

fn main() {
    play_asciijump_in_terminal();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_asciijump_in_terminal() {
    let (stdout, stdin) = initialize_terminal_io();
    let mut game = default_game();
    // TODO where we input height and width as args, can we make it a struct so labelled instead of just
    // guessing?
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = BaseDisplay {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(cursor::FollowPlayerYCursorStrategy::new()),
    };
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut display);

    runtime.start();
}
