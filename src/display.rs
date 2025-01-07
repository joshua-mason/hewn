use std::io::Stdout;
use std::io::Write;
use std::iter::zip;
use termion::raw::RawTerminal;

use crate::engine::game_object::Locate;
use crate::game::Game;

pub struct Display {
    stdout: RawTerminal<Stdout>,
    view_cursor: u16,
    screen_height: u16,
}

impl Display {
    pub fn new(stdout: RawTerminal<Stdout>, screen_height: u16) -> Display {
        Display {
            stdout,
            view_cursor: 0,
            screen_height,
        }
    }

    pub fn stdout(&mut self) -> &mut RawTerminal<Stdout> {
        &mut self.stdout
    }

    pub fn view_cursor(&self) -> u16 {
        self.view_cursor
    }

    pub fn screen_height(&self) -> u16 {
        self.screen_height
    }

    fn debug(game: &Game) -> String {
        if let Some(player) = game.get_player_object() {
            format!(
                "v = {:4}, x = {:3}, y = {:3}",
                player.velocity, player.coordinate.x, player.coordinate.y
            )
        } else {
            "".to_string()
        }
    }

    fn player_view(&mut self, game: &Game) -> String {
        if let Some(player) = game.get_player_object() {
            self.update_cursor(player.coordinate.y);
        }
        let levels = self.levels(game);

        let gotos =
            (0..self.screen_height()).map(|height| termion::cursor::Goto(1, height).to_string());
        zip(levels, gotos)
            .map(|(level, goto)| format!("{}{}", level, goto))
            .collect::<String>()
    }

    // very specific to doodlejump
    // although really getting the lines for the console does make sense?
    // what would a good interface be.. ? maybe the two things we would need to know
    // are determing the starting cursor and then rendering the level given the game object
    // and the level index.. ? I guess assuming we know the coords, we can take the game object
    // and then render based on that.. ?
    fn levels(&mut self, game: &Game) -> Vec<String> {
        let mut level_strings: Vec<String> = vec![];
        for height in 0..self.screen_height() {
            let level = self.render_level(game, height);
            level_strings.push(level);
        }
        level_strings
    }

    fn render_level(&mut self, game: &Game, height: u16) -> String {
        let mut level: String = build_string('.', game.width);
        let y_position = self.screen_height() + self.view_cursor() - height;

        // FIXME: can we make this more efficient?
        let platforms = game.get_platforms();
        let platforms_this_level = platforms
            .iter()
            .filter(|platform| platform.get_coords().y == (y_position) as usize)
            .collect::<Vec<_>>();

        for platform in platforms_this_level {
            let platform_str = build_string('=', 3);
            level.replace_range(
                platform.get_coords().x..(platform.get_coords().x + 3),
                &platform_str,
            );
        }

        if let Some(player) = game.get_player_object() {
            if (y_position) == player.coordinate.y as u16 {
                level.replace_range(player.coordinate.x..(player.coordinate.x + 1), "#");
            }
        }
        level
    }

    // quite specific to doodlejump
    fn update_cursor(&mut self, y: usize) {
        let abs_diff = y.abs_diff(self.view_cursor() as usize);
        if abs_diff > 1 && abs_diff < (self.screen_height() as usize - 2_usize) {
            return;
        }
        self.view_cursor = (y as i16 + 3 - self.screen_height() as i16).max(0) as u16;
    }

    // this can be abstracted to the trait
    pub fn next(&mut self, game: &Game) {
        // this needs to be extracted to a function?
        let view = match game.state {
            crate::game::GameState::InGame => self.player_view(game),
            crate::game::GameState::Menu => String::from("Press space to start"),
            crate::game::GameState::Lost(points) => {
                format!("You scored {} points! Press space to start", points)
            }
        };
        // TODO check if we can get a ref here if we are taking a mutable ref
        // from stdout
        let h: u16 = self.screen_height();
        write!(
            self.stdout(),
            "{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            view,
            termion::cursor::Goto(1, h + 2),
            Display::debug(game),
        )
        .unwrap();

        self.stdout().lock().flush().unwrap();
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
        let mut display = Display::new(stdout, 3);

        let view: String = display.levels(&game).join("\n");

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
        let mut display = Display::new(stdout, 10);

        game.get_mut_player_object().unwrap().coordinate.y = 20;
        display.update_cursor(20);
        let view: String = display.levels(&game).join("\n");

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
        let mut display = Display::new(stdout, 10);
        game.get_mut_player_object().unwrap().coordinate.y = 20;
        display.update_cursor(20);
        game.get_mut_player_object().unwrap().coordinate.y = 19;
        display.update_cursor(19);
        let view: String = display.levels(&game).join("\n");

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
