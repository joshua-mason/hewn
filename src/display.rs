const SCREEN_HEIGHT: u16 = 20;

use std::io::Stdout;
use std::io::Write;
use termion::raw::RawTerminal;

use crate::game::Game;
use crate::game::Platform;
use crate::WIDTH;

pub struct Display {
    stdout: RawTerminal<Stdout>,
}

impl Display {
    pub fn next(&mut self, game: &Game) {
        write!(
            self.stdout,
            "{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            Display::player_level(game),
            termion::cursor::Goto(1, SCREEN_HEIGHT + 2),
            Display::debug(game),
        )
        .unwrap();

        self.stdout.lock().flush().unwrap();
    }

    pub fn new(stdout: RawTerminal<Stdout>) -> Display {
        Display { stdout }
    }

    fn debug(game: &Game) -> String {
        format!(
            "v = {:4}, x = {:3}, y = {:3}",
            game.player_velocity, game.player_pos_x, game.player_pos_y
        )
    }

    fn player_level(game: &Game) -> String {
        let mut display_string = "".to_owned();
        for height in 0..(SCREEN_HEIGHT) {
            let mut level = build_string('.', WIDTH);

            // FIXME: can we make this more efficient?
            let platforms_this_level = game
                .platforms
                .iter()
                .filter(|platform| platform.coordinate.y == height as usize)
                .collect::<Vec<_>>();

            for platform in platforms_this_level {
                platform.render(&mut level);
            }

            if ((SCREEN_HEIGHT - height) == game.player_pos_y as u16) {
                level.replace_range(game.player_pos_x..(game.player_pos_x + 1), "#");
            }
            level.push_str(&termion::cursor::Goto(1, height).to_string());
            display_string.push_str(&level);
        }
        display_string
    }
}

trait DisplayGameObject {
    fn render<'a>(&self, row: &'a mut String);
}

impl DisplayGameObject for Platform {
    fn render<'a>(&self, row: &'a mut String) {
        let platform_str = build_string('=', self.length);
        row.replace_range(
            self.coordinate.x..(self.coordinate.x + self.length),
            &platform_str,
        );
    }
}

fn build_string(char: char, length: usize) -> String {
    let whitespaces = std::iter::repeat(char).take(length);
    let whitespaces = Vec::from_iter(whitespaces);
    let level = String::from_iter(whitespaces);
    level
}
