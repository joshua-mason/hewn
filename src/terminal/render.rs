//! View, cursor and renderer.

use crate::{
    ecs::{Entity, PositionComponent},
    terminal::render::cursor::CursorStrategy,
};
use std::{
    io::{Stdout, Write},
    iter::zip,
};
#[cfg(not(target_arch = "wasm32"))]
use termion::raw::RawTerminal;

/// A coordinate in the game world.
#[derive(Debug, PartialEq, Clone)]
pub struct ViewCoordinate {
    pub x: u16,
    pub y: u16,
}

pub struct ScreenDimensions {
    pub x: u16,
    pub y: u16,
}

/// A view of the game world.
pub struct View {
    pub view_cursor: ViewCoordinate,
    pub renderer: Box<dyn Renderer>,
    pub cursor_strategy: Box<dyn CursorStrategy>,
}

impl View {
    pub fn next(&mut self, entities: Vec<&Entity>, debug_string: Option<String>) -> String {
        let renderer = self.renderer.as_mut();
        let strategy = self.cursor_strategy.as_mut();
        let maybe_trackable_entity = entities
            .iter()
            .find(|entity| entity.components.camera_follow.is_some());
        if let Some(entity_to_track) = maybe_trackable_entity {
            let position = entity_to_track.components.position;
            let pos = position
                .as_ref()
                .unwrap_or(&PositionComponent { x: 0, y: 0 });
            let coord = ViewCoordinate { x: pos.x, y: pos.y };
            strategy.update(&mut self.view_cursor, &*renderer, &coord);
        }

        let mut level_strings: Vec<String> = vec![];

        // TODO would it be much faster to just run through the entities and then render them at the relevant
        // indexes in the level strings?
        for height in 0..renderer.screen_height() {
            let mut level: String = build_string('.', renderer.screen_width() as usize);
            let y_position = renderer.screen_height() + self.view_cursor.y - height;
            let cursor_x_position = self.view_cursor.x;

            for entity in &entities {
                let Some(position) = &entity.components.position else {
                    continue;
                };
                let Some(render) = &entity.components.render else {
                    continue;
                };
                let Some(size) = &entity.components.size else {
                    continue;
                };

                let display_char = render.ascii_character;
                if position.y == y_position
                    && position.x >= cursor_x_position
                    && (position.x + size.x) - cursor_x_position <= level.len() as u16
                {
                    let x_displacement = position.x.saturating_sub(cursor_x_position);
                    let render_x_offset = position.x + size.x - cursor_x_position;
                    level.replace_range(
                        (x_displacement as usize)..(render_x_offset as usize),
                        &display_char
                            .encode_utf8(&mut [0; 4])
                            .repeat(size.x as usize),
                    )
                }
            }

            level_strings.push(level);
        }

        let h: u16 = renderer.screen_height();
        let view = renderer.player_view(level_strings);
        renderer.render(debug_string, view, h)
    }
}

/// Player view cursor and strategies.
pub mod cursor {
    use crate::terminal::render::{Renderer, ViewCoordinate};

    pub trait CursorStrategy {
        fn update(
            &mut self,
            cursor: &mut ViewCoordinate,
            renderer: &dyn Renderer,
            coords: &ViewCoordinate,
        );
    }

    #[derive(Default)]
    pub struct StaticCursorStrategy {}

    impl StaticCursorStrategy {
        pub fn new() -> StaticCursorStrategy {
            StaticCursorStrategy {}
        }
    }

    impl CursorStrategy for StaticCursorStrategy {
        fn update(&mut self, _: &mut ViewCoordinate, _: &dyn Renderer, _: &ViewCoordinate) {}
    }

    #[derive(Default)]
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
            cursor: &mut ViewCoordinate,
            renderer: &dyn Renderer,
            coords: &ViewCoordinate,
        ) {
            let y = coords.y;
            let abs_diff = y.abs_diff(cursor.y);
            if abs_diff > 1 && abs_diff < (renderer.screen_height() - 2_u16) {
                return;
            }
            cursor.y =
                (y as i16 + self.offset as i16 - renderer.screen_height() as i16).max(0) as u16;
        }
    }

    #[derive(Default)]
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
            cursor: &mut ViewCoordinate,
            renderer: &dyn Renderer,
            coords: &ViewCoordinate,
        ) {
            let x = coords.x;
            let abs_diff = x.abs_diff(cursor.x);
            if abs_diff > 1 && abs_diff < (renderer.screen_width() - 2_u16) {
                return;
            }
            cursor.x =
                (x as i16 + self.offset as i16 - renderer.screen_width() as i16).max(0) as u16;
        }
    }
}

/// Trait which all renderers must implement.
pub trait Renderer {
    fn screen_height(&self) -> u16;
    fn screen_width(&self) -> u16;
    fn player_view(&mut self, levels: Vec<String>) -> String;
    fn render(&mut self, debug_string: Option<String>, view: String, h: u16) -> String;
}

/// A renderer for the terminal.
#[cfg(not(target_arch = "wasm32"))]
pub struct TerminalRenderer {
    stdout: RawTerminal<Stdout>,
    screen_dimensions: ScreenDimensions,
}

#[cfg(not(target_arch = "wasm32"))]
impl TerminalRenderer {
    pub fn new(
        stdout: RawTerminal<Stdout>,
        screen_dimensions: ScreenDimensions,
    ) -> TerminalRenderer {
        TerminalRenderer {
            stdout,
            screen_dimensions,
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
        // different use case of terminal vs web, but align to the same trait - worth
        // looking at
        view
    }

    fn player_view(&mut self, levels: Vec<String>) -> String {
        let cursor_y_offset = 2; // termion is (1,1)-based
        let terminal_top_index = cursor_y_offset;
        let terminal_bottom_index = self.screen_height() + cursor_y_offset;
        let terminal_goto_commands = (terminal_top_index..terminal_bottom_index).map(|row_idx| {
            let y_position = row_idx;
            termion::cursor::Goto(1, y_position).to_string()
        });
        zip(levels, terminal_goto_commands).fold(String::new(), |mut acc, (level, goto)| {
            acc.push_str(&level);
            acc.push_str(&goto);
            acc
        })
    }

    fn screen_width(&self) -> u16 {
        self.screen_dimensions.x
    }

    fn screen_height(&self) -> u16 {
        self.screen_dimensions.y
    }
}

/// Utility function to build a string of a given character and length.
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
