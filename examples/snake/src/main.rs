mod game;

use game::game_objects::player_character::PlayerCharacter;
use game::game_objects::wall::Wall;
use game::snake;
use game::{HEIGHT, WIDTH};
use hewn::game_object::Coordinate;

#[cfg(not(target_arch = "wasm32"))]
use hewn::{control::TerminalControl, display::BaseDisplay, initialize_terminal, TerminalRenderer};
fn main() {
    play_snake_in_terminal();
}

const SCREEN_WIDTH: u16 = 30;
const SCREEN_HEIGHT: u16 = 25;

#[cfg(not(target_arch = "wasm32"))]
fn play_snake_in_terminal() {
    let (stdout, stdin) = initialize_terminal();
    let mut game = snake::Game::new(WIDTH, HEIGHT);
    let walls = Wall::generate_walls(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_walls(walls);
    game.generate_food();
    let renderer = TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut display = BaseDisplay {
        renderer: Box::new(renderer),
        view_cursor: Coordinate { x: 0, y: 0 },
        cursor_strategy: Box::new(hewn::display::cursor::StaticCursorStrategy::new()),
    };
    let mut control = TerminalControl::new(stdin, &mut game, &mut display);

    control.listen();
}
