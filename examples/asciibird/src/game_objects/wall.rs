use hewn::{
    game_object::{CollisionBox, Coordinate, GameObject},
    view::build_string,
};
use rand::Rng;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Wall {
    pub coordinate: Coordinate,
}

impl Wall {
    pub fn from_tuple(coords: (usize, usize)) -> Wall {
        Wall {
            coordinate: Coordinate {
                x: coords.0,
                y: coords.1,
            },
        }
    }

    #[cfg(test)]
    pub fn from_tuples(tuples: &[(usize, usize)]) -> Vec<Wall> {
        tuples
            .iter()
            .map(|tuple| Wall::from_tuple(*tuple))
            .collect::<Vec<_>>()
    }

    pub fn generate_walls(width: usize, height: usize) -> Vec<Wall> {
        // TODO build walls not walls
        let mut walls: Vec<Wall> = vec![];
        let mut last_wall: usize = 0;
        let mut rng = rand::thread_rng();

        for index in 0..width {
            if last_wall > 8 {
                let y = rng.gen_range(0..(height - 5));
                let height = 5;
                for y in y..(y + height) {
                    walls.push(Wall::from_tuple((index, y)));
                }
                last_wall = 0;
            }

            if rng.gen_range(0..10) == 0 {
                let y = rng.gen_range(0..(height - 5));
                let height: usize = 5;
                for y in y..(y + height) {
                    walls.push(Wall::from_tuple((index, y)));
                }
                last_wall = 0;
            }
            last_wall += 1;
        }

        walls
    }
}

impl GameObject for Wall {
    fn get_collision_box(&self) -> CollisionBox {
        let coords = self.get_coords();

        CollisionBox {
            x: coords.x..(coords.x + 1),
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
    fn display(&self) -> String {
        build_string('\\', 3)
    }
    fn width(&self) -> usize {
        1
    }
    fn priority(&self) -> u8 {
        1
    }
    fn next_step(&mut self) {}
    fn get_coords(&self) -> &Coordinate {
        &self.coordinate
    }
    fn is_player(&self) -> bool {
        false
    }
}
