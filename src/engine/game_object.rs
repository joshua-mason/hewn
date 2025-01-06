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
