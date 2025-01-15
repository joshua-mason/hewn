use crate::engine::{
    build_string,
    game_object::{Collide, CollisionBox, Coordinate, DisplayObject, GameObject, Locate, NextStep},
};
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerCharacter {
    pub coordinate: Coordinate,
    pub velocity: isize,
    pub hit_wall: bool,
}

impl PlayerCharacter {
    pub fn new() -> PlayerCharacter {
        PlayerCharacter {
            coordinate: Coordinate { x: 1, y: 1 },
            velocity: 5,
            hit_wall: false,
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
            hit_wall: false,
        }
    }

    pub fn jump(&mut self) {
        self.velocity = 5;
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

impl Locate for PlayerCharacter {
    fn get_coords(&self) -> &Coordinate {
        &self.coordinate
    }
}

impl Collide for PlayerCharacter {}

impl NextStep for PlayerCharacter {
    fn next_step(&mut self) {
        self.coordinate.y = (self.coordinate.y as isize + self.velocity).max(0) as usize;
        self.coordinate.x += 1;
        self.velocity -= 1;
    }
}

impl DisplayObject for PlayerCharacter {
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

impl GameObject for PlayerCharacter {
    fn get_collision_box(&self) -> CollisionBox {
        let coords = self.get_coords();
        let next_y_coordinate = (coords.y as isize + self.velocity).max(0) as usize;
        CollisionBox {
            x: coords.x..(coords.x + 1),
            y: coords.y..(coords.y + 1),
        }
    }

    fn collide(&mut self, other: &dyn GameObject) {
        self.hit_wall = true;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
