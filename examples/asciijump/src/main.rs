use hewn::{game_object::Coordinate, BaseDisplay};

use game_objects::{platform::Platform, player_character::PlayerCharacter};
#[cfg(not(target_arch = "wasm32"))]
use hewn::{control::TerminalControl, initialize_terminal, TerminalRenderer};

pub mod game;
pub mod game_objects;

fn main() {
    play_asciijump_in_terminal();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_asciijump_in_terminal() {
    use asciijump::game::{HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, WIDTH};
    use hewn::cursor;

    let (stdout, stdin) = initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_platforms(platforms);
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
