use std::ops::Range;

pub trait Locate {
    fn get_coords(&self) -> &Coordinate;
}

pub trait Collide {
    fn get_collision_box(&self) -> CollisionBox;
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct CollisionBox {
    pub x: Range<usize>,
    pub y: Range<usize>,
}

#[derive(Debug)]
pub struct Platform {
    pub coordinate: Coordinate,
    pub length: usize,
}

#[derive(Debug)]
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
}

impl Locate for Platform {
    fn get_coords(&self) -> &Coordinate {
        &self.coordinate
    }
}

impl Collide for Platform {
    fn get_collision_box(&self) -> CollisionBox {
        CollisionBox {
            x: self.coordinate.x..(self.coordinate.x + self.length),
            y: self.coordinate.y..self.coordinate.y,
        }
    }
}

impl Collide for PlayerCharacter {
    fn get_collision_box(&self) -> CollisionBox {
        let next_y_coordinate = (self.coordinate.y as isize + self.velocity) as usize;
        CollisionBox {
            x: self.coordinate.x..(self.coordinate.x),
            y: self.coordinate.y.min(next_y_coordinate)..next_y_coordinate.max(self.coordinate.y),
        }
    }
}

pub fn detect_collision(a: &impl Collide, b: &impl Collide) -> bool {
    let a_collision_box = a.get_collision_box();
    let b_collision_box = b.get_collision_box();
    fn overlapping_1d(a: Range<usize>, b: Range<usize>) -> bool {
        a.end >= b.start && b.end >= a.start
    }
    overlapping_1d(a_collision_box.x, b_collision_box.x)
        && overlapping_1d(a_collision_box.y, b_collision_box.y)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_collision() {
        let platform = Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        };
        let player = PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 5 },
            velocity: 0,
        };

        assert!(detect_collision(&platform, &player));
    }

    #[test]
    fn test_no_collision() {
        let platform = Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        };
        let player = PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: 0,
        };

        assert!(!detect_collision(&platform, &player));
    }

    #[test]
    fn test_velocity_collision() {
        let platform = Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        };
        let player = PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: -1,
        };

        assert!(detect_collision(&platform, &player));
    }
}
