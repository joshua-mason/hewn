pub struct Entity {
    pub id: EntityId,

    pub position_component: Option<Position>,
    pub velocity_component: Option<Velocity>,
}

#[derive(PartialEq, Debug)]
pub struct EntityId(u16);

enum ComponentType {
    position,
    velocity,
}

trait Component {
    const TYPE: ComponentType;
}
pub struct Position {
    x: u16,
    y: u16,
}
impl Component for Position {
    const TYPE: ComponentType = ComponentType::position;
}

pub struct Velocity {
    x: u16,
    y: u16,
}
impl Component for Velocity {
    const TYPE: ComponentType = ComponentType::velocity;
}

pub struct ECS {
    entities: Vec<Entity>,
}

impl ECS {
    pub fn step(&mut self) {
        let velocity_components = self.get_entities_by_component_mut(ComponentType::velocity);
        for c in velocity_components {
            if let Some(position_component) = &mut c.position_component {
                if let Some(velocity_component) = &mut c.velocity_component {
                    position_component.x += velocity_component.x;
                    position_component.y += velocity_component.y;
                }
            }
        }
    }
}

impl ECS {
    pub fn new() -> ECS {
        ECS { entities: vec![] }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn get_entity_by_id(&self, id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_entity_by_id_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn get_entities_by_component(&self, component_type: ComponentType) -> Vec<&Entity> {
        let entities = self
            .entities
            .iter()
            .filter(|e| match component_type {
                ComponentType::position => {
                    if (e.position_component.is_some()) {
                        return true;
                    }
                    false
                }
                ComponentType::velocity => {
                    if (e.velocity_component.is_some()) {
                        return true;
                    }
                    false
                }
            })
            .collect::<Vec<&Entity>>();
        entities
    }

    pub fn get_entities_by_component_mut(
        &mut self,
        component_type: ComponentType,
    ) -> Vec<&mut Entity> {
        let entities = self
            .entities
            .iter_mut()
            .filter(|e| match component_type {
                ComponentType::position => {
                    if (e.position_component.is_some()) {
                        return true;
                    }
                    false
                }
                ComponentType::velocity => {
                    if (e.velocity_component.is_some()) {
                        return true;
                    }
                    false
                }
            })
            .collect::<Vec<&mut Entity>>();
        entities
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initialise_empty_ecs() {
        let ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0)
    }

    #[test]
    fn test_add_entity_with_no_components() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity = Entity {
            id: EntityId(0),
            position_component: None,
            velocity_component: None,
        };
        ecs.add_entity(entity);
        assert_eq!(ecs.entities.len(), 1);
    }

    #[test]
    fn test_get_entity_by_id() {
        let position_component = Position { x: 0, y: 0 };

        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity = Entity {
            id: EntityId(0),
            position_component: Some(position_component),
            velocity_component: None,
        };
        ecs.add_entity(entity);
        assert_eq!(ecs.entities.len(), 1);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(0));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(0))
    }

    #[test]
    fn test_get_entities_by_ids() {
        let entity_1 = Entity {
            id: EntityId(0),
            position_component: Some(Position { x: 0, y: 0 }),
            velocity_component: None,
        };

        let entity_2 = Entity {
            id: EntityId(1),
            position_component: Some(Position { x: 1, y: 1 }),
            velocity_component: None,
        };
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        ecs.add_entity(entity_1);
        ecs.add_entity(entity_2);
        assert_eq!(ecs.entities.len(), 2);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(0));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(0));
        let entity_position = &entity_from_ecs
            .unwrap()
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0);
        assert_eq!(entity_position.y, 0);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(1));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(1));
        let entity_position = &entity_from_ecs
            .unwrap()
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 1);
        assert_eq!(entity_position.y, 1);
    }

    #[test]
    fn test_ecs_step() {
        let entity_1 = Entity {
            id: EntityId(0),
            position_component: Some(Position { x: 0, y: 0 }),
            velocity_component: Some(Velocity { x: 0, y: 0 }),
        };

        let entity_2 = Entity {
            id: EntityId(1),
            position_component: Some(Position { x: 1, y: 1 }),
            velocity_component: Some(Velocity { x: 1, y: 1 }),
        };
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        ecs.add_entity(entity_1);
        ecs.add_entity(entity_2);
        assert_eq!(ecs.entities.len(), 2);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(0));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(0));
        let entity_velocity = &entity_from_ecs
            .unwrap()
            .velocity_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_velocity.x, 0);
        assert_eq!(entity_velocity.y, 0);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(1));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(1));
        let entity_velocity = &entity_from_ecs
            .unwrap()
            .velocity_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_velocity.x, 1);
        assert_eq!(entity_velocity.y, 1);

        ecs.step();

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(0));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(0));
        let entity_position = &entity_from_ecs
            .unwrap()
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0);
        assert_eq!(entity_position.y, 0);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(1));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(1));
        let entity_position = &entity_from_ecs
            .unwrap()
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 2);
        assert_eq!(entity_position.y, 2);
    }
}
