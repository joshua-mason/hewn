pub struct Entity {
    pub id: EntityId,
    pub components: Components,
}

pub struct Components {
    pub position_component: Option<PositionComponent>,
    pub velocity_component: Option<VelocityComponent>,
    pub render_component: Option<RenderComponent>,
    pub size_component: Option<SizeComponent>,
    pub track_component: Option<TrackComponent>,
}

impl Components {
    pub fn new() -> Components {
        Components {
            position_component: None,
            velocity_component: None,
            render_component: None,
            size_component: None,
            track_component: None,
        }
    }
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub struct EntityId(pub u16);

impl Entity {
    pub fn new(id: EntityId) -> Entity {
        Entity {
            id: id,
            components: Components::new(),
        }
    }
}

pub enum ComponentType {
    position,
    velocity,
    render,
    size,
    track,
}

trait Component {
    const TYPE: ComponentType;
}
pub struct PositionComponent {
    pub x: u16,
    pub y: u16,
}
impl Component for PositionComponent {
    const TYPE: ComponentType = ComponentType::position;
}

pub struct VelocityComponent {
    pub x: i16,
    pub y: i16,
}
impl Component for VelocityComponent {
    const TYPE: ComponentType = ComponentType::velocity;
}

pub struct SizeComponent {
    pub x: u16,
    pub y: u16,
}
impl Component for SizeComponent {
    const TYPE: ComponentType = ComponentType::size;
}

pub struct RenderComponent {
    pub ascii_character: char,
}
impl Component for RenderComponent {
    const TYPE: ComponentType = ComponentType::render;
}

pub struct TrackComponent {}
impl Component for TrackComponent {
    const TYPE: ComponentType = ComponentType::track;
}

pub struct ECS {
    next_entity_id: EntityId,
    entities: Vec<Entity>,
}

impl ECS {
    pub fn step(&mut self) {
        fn clamped_add(position: u16, delta: i16) -> u16 {
            let sum = position as i32 + delta as i32;
            if sum < 0 {
                0
            } else if sum > u16::MAX as i32 {
                u16::MAX
            } else {
                sum as u16
            }
        }
        let velocity_components = self.get_entities_by_component_mut(ComponentType::velocity);
        for c in velocity_components {
            let Some(position_component) = &mut c.components.position_component else {
                continue;
            };

            let Some(velocity_component) = &mut c.components.velocity_component else {
                continue;
            };

            if velocity_component.x != 0 {
                position_component.x = clamped_add(position_component.x, velocity_component.x);
            }
            if velocity_component.y != 0 {
                position_component.y = clamped_add(position_component.y, velocity_component.y);
            }
        }
    }
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: vec![],
            next_entity_id: EntityId(0),
        }
    }

    pub fn add_entity_from_components(&mut self, components: Components) -> EntityId {
        let new_entity_id = self.next_entity_id;
        self.next_entity_id = EntityId(new_entity_id.0 + 1);
        let entity = Entity {
            id: new_entity_id,
            components,
        };
        self.entities.push(entity);
        new_entity_id
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
                    if (e.components.position_component.is_some()) {
                        return true;
                    }
                    false
                }
                ComponentType::velocity => {
                    if (e.components.velocity_component.is_some()) {
                        return true;
                    }
                    false
                }
                ComponentType::size => {
                    if (e.components.size_component.is_some()) {
                        return true;
                    }
                    false
                }
                ComponentType::render => {
                    if (e.components.render_component.is_some()) {
                        return true;
                    }
                    false
                }
                ComponentType::track => {
                    if (e.components.render_component.is_some()) {
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
                    if e.components.position_component.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::velocity => {
                    if e.components.velocity_component.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::size => {
                    if e.components.size_component.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::render => {
                    if e.components.render_component.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::track => {
                    if (e.components.render_component.is_some()) {
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

    fn empty_components() -> Components {
        Components {
            position_component: None,
            velocity_component: None,
            render_component: None,
            size_component: None,
            track_component: None,
        }
    }

    #[test]
    fn test_initialise_empty_ecs() {
        let ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0)
    }

    #[test]
    fn test_add_entity_with_no_components() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        ecs.add_entity_from_components(empty_components());
        assert_eq!(ecs.entities.len(), 1);
    }

    #[test]
    fn test_get_entity_by_id() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        ecs.add_entity_from_components(Components::new());
        assert_eq!(ecs.entities.len(), 1);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(0));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(0))
    }

    #[test]
    fn test_get_entities_by_ids() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity_one_id = ecs.add_entity_from_components(Components {
            position_component: Some(PositionComponent { x: 0, y: 0 }),
            velocity_component: None,
            render_component: None,
            size_component: None,
            track_component: None,
        });
        let entity_two_id = ecs.add_entity_from_components(Components {
            position_component: Some(PositionComponent { x: 1, y: 1 }),
            velocity_component: None,
            render_component: None,
            size_component: None,
            track_component: None,
        });
        assert_eq!(ecs.entities.len(), 2);

        let entity_one_from_ecs = ecs.get_entity_by_id(entity_one_id);
        assert_eq!(entity_one_from_ecs.unwrap().id, EntityId(0));
        let entity_position = &entity_one_from_ecs
            .unwrap()
            .components
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0);
        assert_eq!(entity_position.y, 0);

        let entity_two_from_ecs = ecs.get_entity_by_id(entity_two_id);
        assert_eq!(entity_two_from_ecs.unwrap().id, EntityId(1));
        let entity_position = &entity_two_from_ecs
            .unwrap()
            .components
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 1);
        assert_eq!(entity_position.y, 1);
    }

    #[test]
    fn test_ecs_step() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity_one_id = ecs.add_entity_from_components(Components {
            position_component: Some(PositionComponent { x: 0, y: 0 }),
            velocity_component: Some(VelocityComponent { x: 0, y: 0 }),
            render_component: None,
            size_component: None,
            track_component: None,
        });
        let entity_two_id = ecs.add_entity_from_components(Components {
            position_component: Some(PositionComponent { x: 1, y: 1 }),
            velocity_component: Some(VelocityComponent { x: 1, y: 1 }),
            render_component: None,
            size_component: None,
            track_component: None,
        });
        assert_eq!(ecs.entities.len(), 2);

        let entity_from_ecs = ecs.get_entity_by_id(entity_one_id);
        assert_eq!(entity_from_ecs.unwrap().id, entity_one_id);
        let entity_velocity = &entity_from_ecs
            .unwrap()
            .components
            .velocity_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_velocity.x, 0);
        assert_eq!(entity_velocity.y, 0);

        let entity_from_ecs = ecs.get_entity_by_id(entity_two_id);
        assert_eq!(entity_from_ecs.unwrap().id, entity_two_id);
        let entity_velocity = &entity_from_ecs
            .unwrap()
            .components
            .velocity_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_velocity.x, 1);
        assert_eq!(entity_velocity.y, 1);

        ecs.step();

        let entity_from_ecs = ecs.get_entity_by_id(entity_one_id);
        assert_eq!(entity_from_ecs.unwrap().id, entity_one_id);
        let entity_position = &entity_from_ecs
            .unwrap()
            .components
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0);
        assert_eq!(entity_position.y, 0);

        let entity_from_ecs = ecs.get_entity_by_id(entity_two_id);
        assert_eq!(entity_from_ecs.unwrap().id, entity_two_id);
        let entity_position = &entity_from_ecs
            .unwrap()
            .components
            .position_component
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 2);
        assert_eq!(entity_position.y, 2);
    }
}
