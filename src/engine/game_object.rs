use std::{fmt::Debug, ops::Range};

pub trait Locate {
    fn get_coords(&self) -> &Coordinate;
}

pub trait Collide<T>: Locate + Debug {
    fn collide(&mut self, other: &T);
    fn get_collision_box(&self) -> CollisionBox;
}

pub trait NextStep {
    fn next_step(&mut self);
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
