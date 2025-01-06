use crate::engine::game_object::{Collide, CollisionBox, Coordinate, Locate, NextStep};
use platform::Platform;
use player_character::PlayerCharacter;

pub mod platform;
pub mod player_character;

pub mod utils {
    use std::ops::Range;

    use crate::engine::game_object::GameObject;

    pub fn detect_collision(a: &dyn GameObject, b: &dyn GameObject) -> bool {
        let a_collision_box = a.get_collision_box();
        let b_collision_box = b.get_collision_box();
        fn overlapping_1d(a: Range<usize>, b: Range<usize>) -> bool {
            a.end > b.start && b.end > a.start
        }
        overlapping_1d(a_collision_box.x, b_collision_box.x)
            && overlapping_1d(a_collision_box.y, b_collision_box.y)
    }

    pub fn collision_pass(objects: &mut [Box<dyn GameObject>]) {
        for i in 0..objects.len() {
            let (left, rest) = objects.split_at_mut(i + 1);

            // A is &mut Box<dyn GameObject>
            let a = &mut left[i];

            for b in rest {
                // Now upcast references: &mut dyn GameObject -> &mut dyn Collide
                let x = &mut **a;
                let y: &dyn GameObject = &**b;

                if detect_collision(x, y) {
                    x.collide(y);
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
        let platform = (platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        });
        let player = (player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 5 },
            velocity: 0,
        });

        assert!(utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_no_collision() {
        let platform = (platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        });
        let player = (player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: 0,
        });

        assert!(!utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_velocity_collision() {
        let platform = (platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 10,
        });
        let player = (player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: -1,
        });

        assert!(utils::detect_collision(&platform, &player));
    }
}
