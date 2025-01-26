use crate::engine::{
    build_string,
    game_object::{CollisionBox, Coordinate},
    GameObject,
};
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerCharacter {
    pub coordinate: Coordinate,
    pub velocity: isize,
}

impl PlayerCharacter {
    pub fn new() -> PlayerCharacter {
        PlayerCharacter {
            coordinate: Coordinate { x: 1, y: 1 },
            velocity: 5,
        }
    }

    #[cfg(test)]
    pub fn from_tuple(tuple: (usize, usize, isize)) -> PlayerCharacter {
        PlayerCharacter {
            coordinate: Coordinate {
                x: tuple.0,
                y: tuple.1,
            },
            velocity: tuple.2,
        }
    }

    pub fn move_left(&mut self) {
        self.coordinate.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.coordinate.x += 1;
    }

    pub fn reset(&mut self) {
        self.coordinate.x = 1;
        self.coordinate.y = 1;
        self.velocity = 5;
    }
}

impl Default for PlayerCharacter {
    fn default() -> Self {
        Self::new()
    }
}

impl GameObject for PlayerCharacter {
    fn get_collision_box(&self) -> CollisionBox {
        let coords = self.get_coords();
        let next_y_coordinate = (coords.y as isize + self.velocity).max(0) as usize;
        CollisionBox {
            x: coords.x..(coords.x + 1),
            y: coords.y.min(next_y_coordinate)..next_y_coordinate.max(coords.y),
        }
    }

    fn collide(&mut self, other: &dyn GameObject) {
        // TODO maybe we should check the velocity elsewhere.. ?
        if self.velocity < 1 {
            self.velocity = 5;
            self.coordinate.y = other.get_coords().y;
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    fn get_coords(&self) -> &Coordinate {
        &self.coordinate
    }
    fn next_step(&mut self) {
        self.coordinate.y = (self.coordinate.y as isize + self.velocity).max(0) as usize;
        self.velocity -= 1;
    }
    fn display(&self) -> String {
        build_string('#', 1)
    }
    fn width(&self) -> usize {
        1
    }
    fn priority(&self) -> u8 {
        0
    }
}
