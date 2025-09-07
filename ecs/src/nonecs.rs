use std::time::Duration;

/// A simple struct representing an object with position and velocity.
#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Object {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Object { x, y, vx, vy }
    }
}

/// A container for all objects in the "world".
#[derive(Default)]
pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

/// Updates the positions of all objects in the world based on their velocities and the time delta.
pub fn update_positions(world: &mut World, dt: Duration) {
    let time_s = dt.as_secs_f32();
    for obj in &mut world.objects {
        obj.x = obj.vx.mul_add(time_s, obj.x);
        obj.y = obj.vy.mul_add(time_s, obj.y);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_world() {
        let world = World::new();
        assert_eq!(world.objects.len(), 0);
    }

    #[test]
    fn test_add_object() {
        let mut world = World::new();
        world.add_object(Object::new(0.1, 0.2, 0.3, 0.4));
        assert_eq!(world.objects.len(), 1);
        let obj = world.objects.first().unwrap();
        assert_eq!(obj.x, 0.1);
        assert_eq!(obj.y, 0.2);
        assert_eq!(obj.vx, 0.3);
        assert_eq!(obj.vy, 0.4);
    }

    #[test]
    fn test_update_positions() {
        let mut world = World::new();
        world.add_object(Object::new(0.1, 0.2, 0.3, 0.4));
        update_positions(&mut world, Duration::from_secs_f32(1.0));
        let obj = world.objects.first().unwrap();
        assert_eq!(obj.x, 0.4);
        assert_eq!(obj.y, 0.6);
        assert_eq!(obj.vx, 0.3);
        assert_eq!(obj.vy, 0.4);
    }
}
