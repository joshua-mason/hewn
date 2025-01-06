use std::any::Any;

use rand::Rng;

use crate::engine::game_object::{Collide, CollisionBox, Coordinate, GameObject, Locate, NextStep};

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

impl Collide for Platform {}

impl NextStep for Platform {
    fn next_step(&mut self) {}
}

impl GameObject for Platform {
    fn get_collision_box(&self) -> CollisionBox {
        let coords = self.get_coords();

        CollisionBox {
            x: coords.x..(coords.x + self.length),
            y: coords.y..(coords.y + 1),
        }
    }

    fn collide(&mut self, _: &dyn GameObject) {}
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
