use rand::prelude::*;
use std::ops::Range;

pub trait Locate {
    fn get_coords(&self) -> &Coordinate;
}

pub trait Collide: Locate {
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

pub mod player_character {
    use super::CollisionBox;

    use super::Collide;

    use super::Coordinate;
    use super::Locate;

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

        pub fn from_tuple(tuple: (usize, usize, isize)) -> PlayerCharacter {
            PlayerCharacter {
                coordinate: Coordinate {
                    x: tuple.0,
                    y: tuple.1,
                },
                velocity: tuple.2,
            }
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

    impl Collide for PlayerCharacter {
        fn get_collision_box(&self) -> CollisionBox {
            let coords = self.get_coords();
            let next_y_coordinate = (coords.y as isize + self.velocity).max(0) as usize;
            CollisionBox {
                x: coords.x..(coords.x),
                y: coords.y.min(next_y_coordinate)..next_y_coordinate.max(coords.y),
            }
        }
    }
}

pub mod utils {
    use std::ops::Range;

    use super::Collide;

    pub fn detect_collision(a: &impl Collide, b: &impl Collide) -> bool {
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

    #[derive(Debug)]
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

    impl Collide for Platform {
        fn get_collision_box(&self) -> CollisionBox {
            let coords = self.get_coords();

            CollisionBox {
                x: coords.x..(coords.x + self.length - 1),
                y: coords.y..coords.y,
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_collision() {
        let platform = platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        };
        let player = player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 5 },
            velocity: 0,
        };

        assert!(utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_no_collision() {
        let platform = platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        };
        let player = player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: 0,
        };

        assert!(!utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_velocity_collision() {
        let platform = platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        };
        let player = player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: -1,
        };

        assert!(utils::detect_collision(&platform, &player));
    }
}
