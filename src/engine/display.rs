use crate::engine::display::cursor::CursorStrategy;
use crate::engine::game_object::utils;

use super::game_object::Coordinate;
use super::game_object::GameObject;

pub struct BaseDisplay {
    pub view_cursor: Coordinate,
    pub renderer: Box<dyn Renderer>,
    pub cursor_strategy: Box<dyn CursorStrategy>,
}
use std::{
    io::{Stdout, Write},
    iter::zip,
};
#[cfg(not(target_arch = "wasm32"))]
use termion::raw::RawTerminal;

impl BaseDisplay {
    pub fn next(
        &mut self,
        game_objects: &[Box<dyn GameObject>],
        debug_string: Option<String>,
    ) -> String {
        let renderer = self.renderer.as_mut();
        let strategy = self.cursor_strategy.as_mut();
        if let Some(player_object) = utils::take_player_object(game_objects) {
            strategy.update(&mut self.view_cursor, &*renderer, player_object);
        }

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

        let h: u16 = renderer.screen_height();
        // in the renderer should we just combine the two methods below?
        let view = renderer.player_view(level_strings);
        renderer.render(debug_string, view, h)
    }
}

pub mod cursor {
    use crate::engine::{game_object::Coordinate, GameObject, Renderer};

    pub trait CursorStrategy {
        fn update(
            &mut self,
            cursor: &mut Coordinate,
            renderer: &dyn Renderer,
            player_object: &dyn GameObject,
        );
    }

    pub struct StaticCursorStrategy {}

    impl StaticCursorStrategy {
        pub fn new() -> StaticCursorStrategy {
            StaticCursorStrategy {}
        }
    }

    impl CursorStrategy for StaticCursorStrategy {
        fn update(&mut self, _: &mut Coordinate, _: &dyn Renderer, _: &dyn GameObject) {}
    }

    pub struct FollowPlayerYCursorStrategy {
        offset: usize,
    }

    impl FollowPlayerYCursorStrategy {
        pub fn new() -> FollowPlayerYCursorStrategy {
            FollowPlayerYCursorStrategy { offset: 3 }
        }
    }

    impl CursorStrategy for FollowPlayerYCursorStrategy {
        fn update(
            &mut self,
            cursor: &mut Coordinate,
            renderer: &dyn Renderer,
            player_object: &dyn GameObject,
        ) {
            let y = player_object.get_coords().y;
            let abs_diff = y.abs_diff(cursor.y);
            if abs_diff > 1 && abs_diff < (renderer.screen_height() as usize - 2_usize) {
                return;
            }
            cursor.y =
                (y as i16 + self.offset as i16 - renderer.screen_height() as i16).max(0) as usize;
        }
    }

    pub struct FollowPlayerXCursorStrategy {
        offset: usize,
    }

    impl FollowPlayerXCursorStrategy {
        pub fn new() -> FollowPlayerXCursorStrategy {
            FollowPlayerXCursorStrategy { offset: 16 }
        }
    }

    impl CursorStrategy for FollowPlayerXCursorStrategy {
        fn update(
            &mut self,
            cursor: &mut Coordinate,
            renderer: &dyn Renderer,
            player_object: &dyn GameObject,
        ) {
            let x = player_object.get_coords().x;
            let abs_diff = x.abs_diff(cursor.x);
            if abs_diff > 1 && abs_diff < (renderer.screen_height() as usize - 2_usize) {
                return;
            }
            cursor.x =
                (x as i16 + self.offset as i16 - renderer.screen_width() as i16).max(0) as usize;
        }
    }
}
pub trait Renderer {
    fn screen_height(&self) -> u16;
    fn screen_width(&self) -> u16;
    fn player_view(&mut self, levels: Vec<String>) -> String;
    fn render(&mut self, debug_string: Option<String>, view: String, h: u16) -> String;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct TerminalRenderer {
    stdout: RawTerminal<Stdout>,
    screen_height: u16,
    screen_width: u16,
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
impl Renderer for TerminalRenderer {
    fn render(&mut self, debug_string: Option<String>, view: String, h: u16) -> String {
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
        // TODO unused return value as we flush to the stdout in terminal renderer
        view
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

pub struct WebRenderer {
    screen_height: u16,
    screen_width: u16,
}

impl WebRenderer {
    pub fn new(screen_height: u16, screen_width: u16) -> WebRenderer {
        WebRenderer {
            screen_height,
            screen_width,
        }
    }
}

impl Renderer for WebRenderer {
    fn render(&mut self, debug_string: Option<String>, view: String, h: u16) -> String {
        // TODO unused return value as we just pass through the view
        view
    }

    fn player_view(&mut self, levels: Vec<String>) -> String {
        // this is a hack to conform to the terminal renderer interface
        // TODO: possible to output this as a different type? e.g. just arrays... or better
        // to keep consistent output and handle the string interpretation in client
        // as a quasi custom data structure ? Not in long run but maybe right now.
        let separator = (0..self.screen_height()).map(|_| "|");
        zip(levels, separator).fold(String::new(), |mut acc, (level, goto)| {
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
