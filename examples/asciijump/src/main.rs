use asciijump::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::cursor;
#[cfg(not(target_arch = "wasm32"))]
use hewn::{control::TerminalControl, initialize_terminal, TerminalRenderer};
use hewn::{game_object::Coordinate, BaseDisplay};

use crate::game::default;

pub mod game;
pub mod game_objects;

fn main() {
    play_asciijump_in_terminal();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_asciijump_in_terminal() {
    let (stdout, stdin) = initialize_terminal();
    let mut game = default();
    // TODO where we input height and width as args, can we make it a struct so labelled instead of just
    // guessing?
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = BaseDisplay {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(cursor::FollowPlayerYCursorStrategy::new()),
    };
    let mut control = TerminalControl::new(stdin, &mut game, &mut display);

    control.listen();
}
