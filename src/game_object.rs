use platform::Platform;
use player_character::PlayerCharacter;
use rand::prelude::*;
use std::{fmt::Debug, ops::Range};

pub trait Locate {
    fn get_coords(&self) -> &Coordinate;
}

pub trait Collide<T>: Locate + Debug {
    fn collide(&mut self, other: &T);
    fn get_collision_box(&self) -> CollisionBox;
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameObject {
    PlayerCharacter(PlayerCharacter),
    Platform(Platform),
}
impl GameObject {
    pub fn next_step(&mut self) {
        match self {
            GameObject::PlayerCharacter(player_character) => player_character.next_step(),
            GameObject::Platform(platform) => platform.next_step(),
        }
    }
}

impl Locate for GameObject {
    fn get_coords(&self) -> &Coordinate {
        match self {
            GameObject::PlayerCharacter(player_character) => &player_character.coordinate,
            GameObject::Platform(platform) => &platform.coordinate,
        }
    }
}

trait NextStep {
    fn next_step(&mut self);
}

impl NextStep for GameObject {
    fn next_step(&mut self) {
        self.next_step();
    }
}

impl Collide<GameObject> for GameObject {
    fn collide(&mut self, other: &GameObject) {
        match (self, other) {
            (GameObject::PlayerCharacter(player), GameObject::Platform(platform)) => {
                player.collide(other);
            }
            (GameObject::Platform(platform), GameObject::PlayerCharacter(player)) => {
                platform.collide(other);
            }
            _ => {}
        }
    }

    fn get_collision_box(&self) -> CollisionBox {
        match self {
            GameObject::PlayerCharacter(player_character) => {
                let coords = player_character.get_coords();
                let next_y_coordinate =
                    (coords.y as isize + player_character.velocity).max(0) as usize;
                CollisionBox {
                    x: coords.x..(coords.x),
                    y: coords.y.min(next_y_coordinate)..next_y_coordinate.max(coords.y),
                }
            }
            GameObject::Platform(platform) => {
                let coords = platform.get_coords();

                CollisionBox {
                    x: coords.x..(coords.x + platform.length - 1),
                    y: coords.y..coords.y,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct CollisionBox {
    pub x: Range<usize>,
    pub y: Range<usize>,
}

pub mod player_character {
    use super::Collide;
    use super::CollisionBox;
    use super::Coordinate;
    use super::GameObject;
    use super::Locate;
    use super::NextStep;

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

    impl Locate for PlayerCharacter {
        fn get_coords(&self) -> &Coordinate {
            &self.coordinate
        }
    }

    impl Collide<GameObject> for PlayerCharacter {
        fn get_collision_box(&self) -> CollisionBox {
            let coords = self.get_coords();
            let next_y_coordinate = (coords.y as isize + self.velocity).max(0) as usize;
            CollisionBox {
                x: coords.x..(coords.x),
                y: coords.y.min(next_y_coordinate)..next_y_coordinate.max(coords.y),
            }
        }

        fn collide(&mut self, other: &GameObject) {
            // TODO maybe we should check the velocity elsewhere.. ?
            if (self.velocity < 1) {
                self.velocity = 5;
                self.coordinate.y = other.get_coords().y;
            }
        }
    }

    impl NextStep for PlayerCharacter {
        fn next_step(&mut self) {
            self.coordinate.y = (self.coordinate.y as isize + self.velocity).max(0) as usize;
            self.velocity -= 1;
        }
    }
}

pub mod utils {
    use std::ops::Range;

    use super::{Collide, GameObject};

    pub fn detect_collision(a: &GameObject, b: &GameObject) -> bool {
        let a_collision_box = a.get_collision_box();
        let b_collision_box = b.get_collision_box();
        fn overlapping_1d(a: Range<usize>, b: Range<usize>) -> bool {
            a.end >= b.start && b.end >= a.start
        }
        overlapping_1d(a_collision_box.x, b_collision_box.x)
            && overlapping_1d(a_collision_box.y, b_collision_box.y)
    }
}

pub mod platform {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Platform {
        pub coordinate: Coordinate,
        pub length: usize,
    }

    impl Platform {
        pub fn from_tuple(coords: (usize, usize)) -> Platform {
            Platform {
                coordinate: Coordinate {
                    x: coords.0,
                    y: coords.1,
                },
                length: 3,
            }
        }

        pub fn from_tuples(tuples: &[(usize, usize)]) -> Vec<Platform> {
            tuples
                .iter()
                .map(|tuple| Platform::from_tuple(*tuple))
                .collect::<Vec<_>>()
        }

        pub fn generate_platforms(width: usize, height: usize) -> Vec<Platform> {
            let mut platforms: Vec<Platform> = vec![];
            let mut last_platform: usize = 0;
            let mut rng = rand::thread_rng();

            for index in 0..height {
                if last_platform > 8 {
                    let x = rng.gen_range(0..(width - 3));
                    platforms.push(Platform::from_tuple((x, index)));
                    last_platform = 0;
                }

                if rng.gen_range(0..10) == 0 {
                    let x = rng.gen_range(0..(width - 3));
                    platforms.push(Platform::from_tuple((x, index)));
                    last_platform = 0;
                }
                let y: f64 = rng.gen(); // generates a float between 0 and 1
                last_platform += 1;
            }

            platforms
        }
    }

    impl Locate for Platform {
        fn get_coords(&self) -> &Coordinate {
            &self.coordinate
        }
    }

    impl Collide<GameObject> for Platform {
        fn get_collision_box(&self) -> CollisionBox {
            let coords = self.get_coords();

            CollisionBox {
                x: coords.x..(coords.x + self.length - 1),
                y: coords.y..coords.y,
            }
        }

        fn collide(&mut self, _: &GameObject) {}
    }

    impl NextStep for Platform {
        fn next_step(&mut self) {}
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_collision() {
        let platform = GameObject::Platform(platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        });
        let player = GameObject::PlayerCharacter(player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 5 },
            velocity: 0,
        });

        assert!(utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_no_collision() {
        let platform = GameObject::Platform(platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        });
        let player = GameObject::PlayerCharacter(player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: 0,
        });

        assert!(!utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_velocity_collision() {
        let platform = GameObject::Platform(platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        });
        let player = GameObject::PlayerCharacter(player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: -1,
        });

        assert!(utils::detect_collision(&platform, &player));
    }
}
