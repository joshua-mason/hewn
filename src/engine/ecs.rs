struct Entity {
    pub id: u16,

    // Simpler option here is to strictly set components in the engine - position, velocity
    // More complicated Box dyn etc option is to allow custom components
    // simpler option just as Option<Position>; Option<Velocity>;
    // more complex option is Vec<Box<dyn Component>>... open questions include:
    // how do we prevent adding multiple of the same component in this system?
    // is it efficient?
    pub components: Vec<Box<dyn Component>>,
}

trait Component {
    fn id(&self) -> u16;
}

pub struct ECS {
    entities: Vec<Entity>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS { entities: vec![] }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn get_entity_by_id(&self, id: u16) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_entity_by_id_mut(&mut self, id: u16) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn get_entities_by_component_id(&self, id: u16) -> Vec<&Entity> {
        let entities = self
            .entities
            .iter()
            .filter(|e| {
                let component = e.components.iter().find(|c| c.id() == id);
                if let (c) = component {
                    return true;
                }
                return false;
            })
            .collect::<Vec<&Entity>>();
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
            components: vec![],
            id: 0,
        };
        ecs.add_entity(entity);
        assert_eq!(ecs.entities.len(), 1);
    }

    #[test]
    fn test_add_entity_with_one_component() {
        struct Position {}
        impl Position {
            const ID: u16 = 0;
        }
        impl Component for Position {
            fn id(&self) -> u16 {
                Position::ID
            }
        }

        let position_component = Position {};

        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity = Entity {
            components: vec![Box::new(position_component)],
            id: 0,
        };
        ecs.add_entity(entity);
        assert_eq!(ecs.entities.len(), 1);
    }

    #[test]
    fn test_get_entity_with_one_component() {
        struct Position {}
        impl Position {
            const ID: u16 = 0;
        }
        impl Component for Position {
            fn id(&self) -> u16 {
                Position::ID
            }
        }

        let position_component = Position {};

        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity = Entity {
            components: vec![Box::new(position_component)],
            id: 0,
        };
        ecs.add_entity(entity);
        assert_eq!(ecs.entities.len(), 1);

        let entity_from_ecs = ecs.get_entity_by_id(0);
        assert_eq!(entity_from_ecs.unwrap().id, 0)
    }

    #[test]
    fn test_get_entities_with_one_component() {
        struct Position {}
        impl Position {
            const ID: u16 = 0;
        }
        impl Component for Position {
            fn id(&self) -> u16 {
                Position::ID
            }
        }

        let position_component_1 = Position {};
        let position_component_2 = Position {};

        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity = Entity {
            components: vec![
                Box::new(position_component_1),
                Box::new(position_component_2),
            ],
            id: 0,
        };
        ecs.add_entity(entity);
        ecs.add_entity(entity);
        assert_eq!(ecs.entities.len(), 1);

        let entity_from_ecs = ecs.get_entity_by_id(0);
        assert_eq!(entity_from_ecs.unwrap().id, 0)
    }
}
