use crate::engine::{
    control::TerminalControl, cursor, game_object::Coordinate, initialize_terminal, BaseDisplay,
    TerminalRenderer,
};
use game_objects::{player_character::PlayerCharacter, wall::Wall};

pub mod game;
pub mod game_objects;

const WIDTH: usize = 1000;
const HEIGHT: usize = 30;
const SCREEN_WIDTH: u16 = 50;
const SCREEN_HEIGHT: u16 = 30;

pub fn play_asciibird() {
    let (stdout, stdin) = initialize_terminal();
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
