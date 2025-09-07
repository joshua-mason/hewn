use std::time::Duration;

pub struct EntityId(pub u32);

struct Entity {
    id: EntityId,
}

pub struct Position {
    x: f32,
    y: f32,
}

impl From<(f32, f32)> for Position {
    fn from(value: (f32, f32)) -> Self {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}

pub struct Velocity {
    x: f32,
    y: f32,
}

impl From<(f32, f32)> for Velocity {
    fn from(value: (f32, f32)) -> Self {
        Velocity {
            x: value.0,
            y: value.1,
        }
    }
}

pub struct ECS {
    // components: HashMap<String, Vec<Component>>,
    position_components: Vec<Position>,
    velocity_components: Vec<Velocity>,
    entities: Vec<EntityId>,
    next_entity_id: u32,
}

impl ECS {
    pub fn new() -> ECS {
        // let mut components: HashMap<String, Vec<Component>> = HashMap::new();
        // components.insert("Position".to_owned(), vec![]);
        // components.insert("Velocity".to_owned(), vec![]);
        ECS {
            entities: vec![],
            position_components: vec![],
            velocity_components: vec![],
            next_entity_id: 0,
        }
    }

    pub fn add_entity(&mut self, pos: Position, vel: Velocity) -> &EntityId {
        self.position_components.push(pos);
        self.velocity_components.push(vel);
        let entity_id = EntityId(self.next_entity_id);
        self.entities.push(entity_id);
        self.next_entity_id += 1;
        self.entities.last().unwrap()
    }
}

pub fn run_update_positions_system(ecs: &mut ECS, dt: Duration) {
    let time_s = dt.as_secs_f32();
    for (pos, vel) in ecs
        .position_components
        .iter_mut()
        .zip(ecs.velocity_components.iter())
    {
        pos.x = vel.x.mul_add(time_s, pos.x);
        pos.y = vel.y.mul_add(time_s, pos.y);
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{run_update_positions_system, ECS};

    #[test]
    fn test_init_ecs() {
        let ecs = ECS::new();
        let positions = ecs.position_components;
        assert_eq!(positions.len(), 0);
        let velocity = ecs.velocity_components;
        assert_eq!(velocity.len(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut ecs = ECS::new();
        ecs.add_entity((0.1, 0.2).into(), (0.3, 0.4).into());
        let positions = ecs.position_components;
        assert_eq!(positions.len(), 1);
        assert_eq!(positions.first().expect("Position should be added").x, 0.1);
        assert_eq!(positions.first().expect("Position should be added").y, 0.2);
        let velocity = ecs.velocity_components;
        assert_eq!(velocity.len(), 1);
        assert_eq!(velocity.first().expect("Position should be added").x, 0.3);
        assert_eq!(velocity.first().expect("Position should be added").y, 0.4);
    }

    #[test]
    fn test_run_update_positions_system() {
        let mut ecs = ECS::new();
        ecs.add_entity((0.1, 0.2).into(), (0.3, 0.4).into());
        run_update_positions_system(&mut ecs, Duration::from_secs_f32(1.0));
        let positions = ecs.position_components;
        assert_eq!(positions.len(), 1);
        assert_eq!(positions.first().expect("Position should be added").x, 0.4);
        assert_eq!(positions.first().expect("Position should be added").y, 0.6);
        let velocity = ecs.velocity_components;
        assert_eq!(velocity.len(), 1);
        assert_eq!(velocity.first().expect("Position should be added").x, 0.3);
        assert_eq!(velocity.first().expect("Position should be added").y, 0.4);
    }
}
