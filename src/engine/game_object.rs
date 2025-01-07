use std::{any::Any, fmt::Debug, ops::Range};

pub trait Locate {
    fn get_coords(&self) -> &Coordinate;
}

pub trait Collide: Locate + Debug {}

pub trait NextStep {
    fn next_step(&mut self);
}

pub trait GameObject: Collide + NextStep + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn collide(&mut self, other: &dyn GameObject);
    fn get_collision_box(&self) -> CollisionBox;
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

pub fn try_get_concrete_type<T: Any>(abc: &dyn GameObject) -> Option<&T> {
    abc.as_any().downcast_ref::<T>()
}
pub fn try_get_mut_concrete_type<T: Any>(abc: &mut dyn GameObject) -> Option<&mut T> {
    abc.as_mut_any().downcast_mut()
}

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

    #[derive(Debug)]
    struct TestGameObject {
        coords: Coordinate,
    }

    impl Locate for TestGameObject {
        fn get_coords(&self) -> &Coordinate {
            return &self.coords;
        }
    }

    impl Collide for TestGameObject {}

    impl GameObject for TestGameObject {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_mut_any(&mut self) -> &mut dyn Any {
            self
        }
        fn collide(&mut self, _: &dyn GameObject) {}
        fn get_collision_box(&self) -> CollisionBox {
            CollisionBox {
                x: self.coords.x..self.coords.x,
                y: self.coords.y..self.coords.y,
            }
        }
    }

    impl NextStep for TestGameObject {
        fn next_step(&mut self) {}
    }

    #[test]
    fn test_collision() {
        let platform = (TestGameObject {
            coords: Coordinate { x: 5, y: 5 },
        });
        let player = (TestGameObject {
            coords: Coordinate { x: 5, y: 5 },
        });

        assert!(utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_no_collision() {
        let platform = (TestGameObject {
            coords: Coordinate { x: 5, y: 5 },
        });
        let player = (TestGameObject {
            coords: Coordinate { x: 7, y: 6 },
        });

        assert!(!utils::detect_collision(&platform, &player));
    }
}
