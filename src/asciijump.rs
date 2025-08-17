use crate::engine::{game_object::Coordinate, BaseDisplay};

#[cfg(not(target_arch = "wasm32"))]
use crate::engine::{control::TerminalControl, initialize_terminal, TerminalRenderer};
use game_objects::{platform::Platform, player_character::PlayerCharacter};

pub mod game;
pub mod game_objects;

const WIDTH: usize = 10;
const HEIGHT: usize = 500;
const SCREEN_WIDTH: u16 = 10;
const SCREEN_HEIGHT: u16 = 20;

#[cfg(not(target_arch = "wasm32"))]
pub fn play_asciijump_in_terminal() {
    use crate::engine::cursor;

    let (stdout, stdin) = initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_platforms(platforms);
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = BaseDisplay {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(cursor::FollowPlayerYCursorStrategy::new()),
    };
    let mut control = TerminalControl::new(stdin, &mut game, &mut display);

    control.listen();
}
