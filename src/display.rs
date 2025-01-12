use std::io::Stdout;
use std::iter::zip;
use termion::raw::RawTerminal;

use crate::engine::display::BaseDisplay;
use crate::engine::game_object::GameObject;
use crate::engine::game_object::Locate;
use crate::game::take_platforms;
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

    // very specific to doodlejump
    // although really getting the lines for the console does make sense?
    // what would a good interface be.. ? maybe the two things we would need to know
    // are determing the starting cursor and then rendering the level given the game object
    // and the level index.. ? I guess assuming we know the coords, we can take the game object
    // and then render based on that.. ?
    fn levels(&mut self, game_objects: &[Box<dyn GameObject>]) -> Vec<String> {
        let mut level_strings: Vec<String> = vec![];
        for height in 0..self.screen_height() {
            let level = self.render_level(game_objects, height);
            level_strings.push(level);
        }
        level_strings
    }

    fn render_level(&mut self, game_objects: &[Box<dyn GameObject>], height: u16) -> String {
        let mut level: String = build_string('.', self.screen_width as usize);
        let y_position = self.screen_height() + self.view_cursor() - height;

        // FIXME: can we make this more efficient?
        let platforms = take_platforms(game_objects);
        let platforms_this_level = platforms
            .iter()
            .filter(|platform| platform.get_coords().y == (y_position as usize))
            .collect::<Vec<_>>();

        for platform in platforms_this_level {
            let platform_str = build_string('=', 3);
            level.replace_range(
                platform.get_coords().x..(platform.get_coords().x + 3),
                &platform_str,
            );
        }

        if let Some(player) = take_player_object(game_objects) {
            if (y_position) == player.coordinate.y as u16 {
                level.replace_range(player.coordinate.x..(player.coordinate.x + 1), "#");
            }
        }
        level
    }

    // TODO how do we get this from the game?
    // ORRRRR we actually create separate display objects for different scenes.. ? could be a good interface for it! Maybe a macro or something for it
    // interesting questions..
    // fn game_state(&self) -> &crate::game::GameState {
    //     &crate::game::GameState::InGame
    // }
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

    fn player_view(&mut self, game_objects: &[Box<dyn GameObject>]) -> String {
        let levels = self.levels(game_objects);

        let gotos =
            (0..self.screen_height()).map(|height| termion::cursor::Goto(1, height).to_string());
        zip(levels, gotos)
            .map(|(level, goto)| format!("{}{}", level, goto))
            .collect::<String>()
    }
}

fn build_string(ch: char, length: usize) -> String {
    ch.to_string().repeat(length)
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

    #[test]
    fn test_build_string() {
        let input = build_string('@', 3);
        assert_eq!(input, "@@@");
    }
}
