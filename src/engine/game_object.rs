//! Game object traits and utils.

use std::{any::Any, fmt::Debug, ops::Range};

pub trait GameObject: Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn collide(&mut self, other: &dyn GameObject);
    fn display(&self) -> String;
    fn get_collision_box(&self) -> CollisionBox;
    fn get_coords(&self) -> &Coordinate;
    fn next_step(&mut self);
    fn priority(&self) -> u8;
    fn width(&self) -> usize;
    fn is_player(&self) -> bool;
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

pub mod utils {
    use crate::engine::game_object::GameObject;
    use std::{any::Any, ops::Range};

    pub fn detect_collision(a: &dyn GameObject, b: &dyn GameObject) -> bool {
        let a_collision_box = a.get_collision_box();
        let b_collision_box = b.get_collision_box();
        fn overlapping_1d(a: Range<usize>, b: Range<usize>) -> bool {
            a.end > b.start && b.end > a.start
        }
        overlapping_1d(a_collision_box.x, b_collision_box.x)
            && overlapping_1d(a_collision_box.y, b_collision_box.y)
    }

    // TODO: this seems to be affected by the order of the objects - probably related to the double dispatch problem?
    pub fn collision_pass(objects: &mut [Box<dyn GameObject>]) {
        process_collisions(objects);
        objects.reverse();
        process_collisions(objects);
        objects.reverse();
    }

    fn process_collisions(objects: &mut [Box<dyn GameObject>]) {
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

    pub fn downcast_refs<T: GameObject>(game_objects: &[Box<dyn GameObject>]) -> Vec<&T> {
        game_objects
            .iter()
            .filter_map(|o| maybe_get_concrete_type::<T>(&**o))
            .collect::<Vec<&T>>()
    }

    pub fn downcast_muts<'a, T: GameObject>(
        game_objects: &'a mut [Box<dyn GameObject>],
    ) -> Vec<&'a mut T> {
        game_objects
            .iter_mut()
            .filter_map(|o| maybe_get_concrete_type_mut::<T>(&mut **o))
            .collect::<Vec<&'a mut T>>()
    }

    pub fn take_game_object<T: GameObject>(game_objects: &[Box<dyn GameObject>]) -> Option<&T> {
        downcast_refs::<T>(game_objects).into_iter().next()
    }

    pub fn take_player_object(game_objects: &[Box<dyn GameObject>]) -> Option<&dyn GameObject> {
        game_objects.iter().find(|o| o.is_player()).map(|o| &**o)
    }

    pub fn maybe_get_concrete_type<T: Any>(abc: &dyn GameObject) -> Option<&T> {
        abc.as_any().downcast_ref::<T>()
    }

    pub fn maybe_get_concrete_type_mut<T: Any>(abc: &mut dyn GameObject) -> Option<&mut T> {
        abc.as_mut_any().downcast_mut()
    }
}

#[cfg(test)]
mod test {

    use super::{
        utils::{collision_pass, downcast_refs},
        *,
    };

    #[derive(Debug)]
    struct TestGameObject {
        coords: Coordinate,
        collisions: u8,
    }

    impl TestGameObject {
        pub fn from_tuple((x, y): (usize, usize)) -> TestGameObject {
            TestGameObject {
                coords: Coordinate { x, y },
                collisions: 0,
            }
        }
    }

    impl GameObject for TestGameObject {
        fn get_coords(&self) -> &Coordinate {
            &self.coords
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_mut_any(&mut self) -> &mut dyn Any {
            self
        }
        fn collide(&mut self, _: &dyn GameObject) {
            self.collisions += 1;
        }
        fn get_collision_box(&self) -> CollisionBox {
            CollisionBox {
                x: self.coords.x..(self.coords.x + 1),
                y: self.coords.y..(self.coords.y + 1),
            }
        }

        fn display(&self) -> String {
            "---".to_owned()
        }
        fn width(&self) -> usize {
            3
        }
        fn priority(&self) -> u8 {
            1
        }
        fn next_step(&mut self) {}
        fn is_player(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_collision() {
        let platform = TestGameObject::from_tuple((5, 5));
        let player = TestGameObject::from_tuple((5, 5));

        assert!(utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_no_collision() {
        let platform = TestGameObject::from_tuple((5, 5));
        let player = TestGameObject::from_tuple((7, 6));

        assert!(!utils::detect_collision(&platform, &player));
    }

    #[test]
    fn test_collision_pass() {
        let mut game_objects: Vec<Box<dyn GameObject>> = vec![
            Box::new(TestGameObject::from_tuple((1, 1))),
            Box::new(TestGameObject::from_tuple((1, 1))),
        ];

        collision_pass(&mut game_objects);

        println!("{:?}", game_objects);

        for game_object in downcast_refs::<TestGameObject>(&game_objects).iter() {
            assert_eq!(game_object.collisions, 1);
        }
    }
}
