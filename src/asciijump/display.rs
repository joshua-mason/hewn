use std::io::Stdout;
use termion::raw::RawTerminal;

use crate::engine::display::BaseDisplay;
use crate::engine::game_object::GameObject;
use crate::game::take_player_object;

pub struct Display {
    stdout: RawTerminal<Stdout>,
    view_cursor: u16,
    screen_height: u16,
    screen_width: u16,
}

impl Display {
    pub fn new(stdout: RawTerminal<Stdout>, screen_height: u16, screen_width: u16) -> Display {
        Display {
            stdout,
            view_cursor: 0,
            screen_height,
            screen_width,
        }
    }
}

impl BaseDisplay for Display {
    fn update_cursor(&mut self, game_objects: &[Box<dyn GameObject>]) {
        if let Some(player_object) = take_player_object(game_objects) {
            let y = player_object.coordinate.y;
            let abs_diff = y.abs_diff(self.view_cursor() as usize);
            if abs_diff > 1 && abs_diff < (self.screen_height() as usize - 2_usize) {
                return;
            }
            self.view_cursor = (y as i16 + 3 - self.screen_height() as i16).max(0) as u16;
        }
    }

    fn stdout(&mut self) -> &mut RawTerminal<Stdout> {
        &mut self.stdout
    }

    fn view_cursor(&self) -> u16 {
        self.view_cursor
    }

    fn screen_height(&self) -> u16 {
        self.screen_height
    }

    fn screen_width(&self) -> u16 {
        self.screen_width
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::Game;
    use std::io::{self};
    use termion::raw::IntoRawMode;

    #[test]
    fn test_starting_display() {
        let game = Game::new(3, 3);
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut display = Display::new(stdout, 3, 3);

        let view: String = display.levels(&game.game_objects).join("\n");

        assert!(
            view == "...
...
.#."
        )
    }

    #[test]
    fn test_player_moved_up() {
        let mut game = Game::new(3, 100);
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut display = Display::new(stdout, 10, 3);

        game.get_mut_player_object().unwrap().coordinate.y = 20;
        display.update_cursor(&game.game_objects);
        let view: String = display.levels(&game.game_objects).join("\n");

        assert!(
            view == "...
...
...
.#.
...
...
...
...
...
..."
        )
    }

    #[test]
    fn test_player_moved_up_and_back_down() {
        let mut game = Game::new(3, 100);
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut display = Display::new(stdout, 10, 3);
        game.get_mut_player_object().unwrap().coordinate.y = 20;
        display.update_cursor(&game.game_objects);
        game.get_mut_player_object().unwrap().coordinate.y = 19;
        display.update_cursor(&game.game_objects);
        let view: String = display.levels(&game.game_objects).join("\n");

        assert!(
            view == "...
...
...
...
.#.
...
...
...
...
..."
        )
    }
}
