mod game;
mod game_objects;

use game::{HEIGHT, WIDTH};
use game_objects::player_character::PlayerCharacter;
use game_objects::wall::Wall;
use hewn::game_object::Coordinate;

#[cfg(not(target_arch = "wasm32"))]
use hewn::{control::TerminalControl, display::BaseDisplay, initialize_terminal, TerminalRenderer};
fn main() {
    play_asciibird_in_terminal();
}

#[cfg(not(target_arch = "wasm32"))]
fn play_asciibird_in_terminal() {
    let (stdout, stdin) = initialize_terminal();

    use asciibird::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
    use hewn::cursor;
    let mut game = game::Game::new();
    let walls = Wall::generate_walls(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_walls(walls);
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = BaseDisplay {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(cursor::FollowPlayerXCursorStrategy::new()),
    };
    let mut control = TerminalControl::new(stdin, &mut game, &mut display);

    control.listen();
}
