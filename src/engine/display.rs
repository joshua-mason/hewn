use super::game_object::Coordinate;
use super::game_object::GameObject;

pub struct BaseDisplay {
    pub view_cursor: Coordinate,
    pub renderer: Box<dyn Renderer>,
}
use std::{
    io::{Stdout, Write},
    iter::zip,
};
use termion::raw::RawTerminal;

impl BaseDisplay {
    pub fn next(&mut self, game_objects: &[Box<dyn GameObject>], debug_string: Option<String>) {
        let renderer = self.renderer.as_mut();

        let mut level_strings: Vec<String> = vec![];
        for height in 0..renderer.screen_height() {
            let mut level: String = build_string('.', renderer.screen_width() as usize);
            let y_position = renderer.screen_height() + self.view_cursor.y as u16 - height;
            let cursor_x_position = self.view_cursor.x;

            for game_object in game_objects {
                let game_object_coords = game_object.get_coords();
                let game_object_width = game_object.width();
                let mut display_string: &str = &game_object.display();
                if display_string.len() > game_object_width {
                    display_string = display_string.split_at(game_object_width).0;
                }
                if game_object_coords.y == (y_position as usize)
                    && game_object_coords.x >= cursor_x_position
                    && game_object_coords.x + game_object_width - cursor_x_position <= level.len()
                {
                    let x_displacement = if cursor_x_position > game_object_coords.x {
                        0
                    } else {
                        game_object_coords.x - cursor_x_position
                    };
                    let render_x_offset =
                        game_object_coords.x + game_object_width - cursor_x_position;
                    level.replace_range((x_displacement)..(render_x_offset), display_string)
                }
            }

            level_strings.push(level);
        }

        let view = renderer.player_view(level_strings);

        let h: u16 = renderer.screen_height();
        renderer.render(debug_string, view, h);
    }
}

pub fn build_string(ch: char, length: usize) -> String {
    ch.to_string().repeat(length)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_build_string() {
        let input = build_string('@', 3);
        assert_eq!(input, "@@@");
    }
}

pub trait Renderer {
    fn screen_height(&self) -> u16;
    fn screen_width(&self) -> u16;
    fn player_view(&mut self, levels: Vec<String>) -> String;
    fn render(&mut self, debug_string: Option<String>, view: String, h: u16);
}

pub struct TerminalRenderer {
    stdout: RawTerminal<Stdout>,
    screen_height: u16,
    screen_width: u16,
}

impl TerminalRenderer {
    pub fn new(
        stdout: RawTerminal<Stdout>,
        screen_height: u16,
        screen_width: u16,
    ) -> TerminalRenderer {
        TerminalRenderer {
            stdout,
            screen_height,
            screen_width,
        }
    }

    fn stdout(&mut self) -> &mut RawTerminal<Stdout> {
        &mut self.stdout
    }
}

impl Renderer for TerminalRenderer {
    fn render(&mut self, debug_string: Option<String>, view: String, h: u16) {
        write!(
            self.stdout(),
            "{}{}{}{}{:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            view,
            termion::cursor::Goto(1, h + 2),
            debug_string
        )
        .unwrap();
        self.stdout().lock().flush().unwrap();
    }

    fn player_view(&mut self, levels: Vec<String>) -> String {
        let gotos =
            (0..self.screen_height()).map(|height| termion::cursor::Goto(1, height).to_string());
        zip(levels, gotos).fold(String::new(), |mut acc, (level, goto)| {
            acc.push_str(&level);
            acc.push_str(&goto);
            acc
        })
    }

    fn screen_height(&self) -> u16 {
        self.screen_height
    }

    fn screen_width(&self) -> u16 {
        self.screen_width
    }
}
