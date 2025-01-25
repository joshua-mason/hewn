use super::game_objects::player_character::PlayerCharacter;
use crate::engine::{
    game_object::{utils::take_game_object, Coordinate},
    BaseDisplay, GameObject,
};
use std::io::Stdout;
use termion::raw::RawTerminal;

pub struct Display {
    stdout: RawTerminal<Stdout>,
    view_cursor: Coordinate,
    screen_height: u16,
    screen_width: u16,
}

impl Display {
    pub fn new(stdout: RawTerminal<Stdout>, screen_height: u16, screen_width: u16) -> Display {
        Display {
            stdout,
            view_cursor: Coordinate { x: 0, y: 0 },
            screen_height,
            screen_width,
        }
    }
}

impl BaseDisplay for Display {
    fn update_cursor(&mut self, game_objects: &[Box<dyn GameObject>]) {
        if let Some(player_object) = take_game_object::<PlayerCharacter>(game_objects) {
            let x = player_object.coordinate.x;
            let abs_diff = x.abs_diff(self.view_cursor().x as usize);
            if abs_diff > 1 && abs_diff < (self.screen_height() as usize - 2_usize) {
                return;
            }
            self.view_cursor.x = (x as i16 + 15 - self.screen_width() as i16).max(0) as usize;
        }
    }

    fn stdout(&mut self) -> &mut RawTerminal<Stdout> {
        &mut self.stdout
    }

    fn view_cursor(&self) -> &Coordinate {
        &self.view_cursor
    }

    fn screen_height(&self) -> u16 {
        self.screen_height
    }

    fn screen_width(&self) -> u16 {
        self.screen_width
    }
}
