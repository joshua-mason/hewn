use rand::Rng;

use crate::engine::game_object::{Collide, CollisionBox, Coordinate, Locate, NextStep};

use super::GameObject;

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
            x: coords.x..(coords.x + self.length),
            y: coords.y..(coords.y + 1),
        }
    }

    fn collide(&mut self, _: &GameObject) {}
}

impl NextStep for Platform {
    fn next_step(&mut self) {}
}
