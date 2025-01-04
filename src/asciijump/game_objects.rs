use crate::engine::game_object::{Collide, CollisionBox, Coordinate, Locate, NextStep};
use platform::Platform;
use player_character::PlayerCharacter;

pub mod platform;
pub mod player_character;

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
    pub fn get_collision_box(&self) -> CollisionBox {
        match self {
            GameObject::PlayerCharacter(pc) => pc.get_collision_box(),
            GameObject::Platform(platform) => platform.get_collision_box(),
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

pub mod utils {
    use std::ops::Range;

    use crate::engine::game_object::Collide;

    use super::GameObject;

    pub fn detect_collision(a: &GameObject, b: &GameObject) -> bool {
        let a_collision_box = a.get_collision_box();
        let b_collision_box = b.get_collision_box();
        fn overlapping_1d(a: Range<usize>, b: Range<usize>) -> bool {
            a.end > b.start && b.end > a.start
        }
        overlapping_1d(a_collision_box.x, b_collision_box.x)
            && overlapping_1d(a_collision_box.y, b_collision_box.y)
    }

    pub fn collision_pass(objects: &mut [GameObject]) {
        for i in 0..objects.len() {
            let (left, rest) = objects.split_at_mut(i + 1);
            let mut a = &mut left[i];

            for mut b in rest {
                if detect_collision(a, b) {
                    match &mut a {
                        GameObject::PlayerCharacter(pc) => pc.collide(b),
                        GameObject::Platform(platform) => platform.collide(b),
                    }
                    // match on the second
                    match &mut b {
                        GameObject::PlayerCharacter(pc) => pc.collide(a),
                        GameObject::Platform(platform) => platform.collide(a),
                    }
                }
            }
        }
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
