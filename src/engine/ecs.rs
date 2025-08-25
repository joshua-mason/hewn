#[derive(Debug)]
pub struct Entity {
    pub id: EntityId,
    pub components: Components,
}

#[derive(Debug)]
pub struct Components {
    pub position: Option<PositionComponent>,
    pub velocity: Option<VelocityComponent>,
    pub render: Option<RenderComponent>,
    pub size: Option<SizeComponent>,
    pub camera_follow: Option<CameraFollow>,
}

impl Components {
    pub fn new() -> Components {
        Components {
            position: None,
            velocity: None,
            render: None,
            size: None,
            camera_follow: None,
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

    pub fn from_tuples(
        id: EntityId,
        pos: (u16, u16),
        vel: (i16, i16),
        size: (u16, u16),
        ascii_character: Option<char>,
        track: bool,
    ) -> Entity {
        Entity {
            id,
            components: Components {
                position: Some(PositionComponent { x: pos.0, y: pos.1 }),
                velocity: Some(VelocityComponent { x: vel.0, y: vel.1 }),
                size: Some(SizeComponent {
                    x: size.0,
                    y: size.1,
                }),
                render: ascii_character.map(|c| RenderComponent { ascii_character: c }),
                camera_follow: if track { Some(CameraFollow {}) } else { None },
            },
        }
    }
}

pub enum ComponentType {
    Position,
    Velocity,
    Render,
    Size,
    CameraFollow,
}

#[allow(dead_code)]
trait Component {
    const TYPE: ComponentType;
}
#[derive(Debug)]
pub struct PositionComponent {
    pub x: u16,
    pub y: u16,
}
impl Component for PositionComponent {
    const TYPE: ComponentType = ComponentType::Position;
}

#[derive(Debug)]
pub struct VelocityComponent {
    pub x: i16,
    pub y: i16,
}
impl Component for VelocityComponent {
    const TYPE: ComponentType = ComponentType::Velocity;
}

#[derive(Debug)]
pub struct SizeComponent {
    pub x: u16,
    pub y: u16,
}
impl Component for SizeComponent {
    const TYPE: ComponentType = ComponentType::Size;
}

#[derive(Debug)]
pub struct RenderComponent {
    pub ascii_character: char,
}
impl Component for RenderComponent {
    const TYPE: ComponentType = ComponentType::Render;
}

#[derive(Debug)]
pub struct CameraFollow {}
impl Component for CameraFollow {
    const TYPE: ComponentType = ComponentType::CameraFollow;
}

pub struct ECS {
    next_entity_id: EntityId,
    entities: Vec<Entity>,
}

impl ECS {
    pub fn step(&mut self) {
        // Consider splitting systems e.g. if we are handling gravity in the future
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
        let velocitys = self.get_entities_by_mut(ComponentType::Velocity);
        for c in velocitys {
            let Some(position) = &mut c.components.position else {
                continue;
            };

            let Some(velocity) = &mut c.components.velocity else {
                continue;
            };

            if velocity.x != 0 {
                position.x = clamped_add(position.x, velocity.x);
            }
            if velocity.y != 0 {
                position.y = clamped_add(position.y, velocity.y);
            }
        }
    }

    pub fn collision_pass(&self) -> Vec<[EntityId; 2]> {
        collisions::collision_pass(&self.entities)
    }
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: vec![],
            next_entity_id: EntityId(0),
        }
    }

    pub fn add_entity_froms(&mut self, components: Components) -> EntityId {
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

    pub fn get_entities_by(&self, component_type: ComponentType) -> Vec<&Entity> {
        let entities = self
            .entities
            .iter()
            .filter(|e| match component_type {
                ComponentType::Position => {
                    if e.components.position.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::Velocity => {
                    if e.components.velocity.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::Size => {
                    if e.components.size.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::Render => {
                    if e.components.render.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::CameraFollow => {
                    if e.components.camera_follow.is_some() {
                        return true;
                    }
                    false
                }
            })
            .collect::<Vec<&Entity>>();
        entities
    }

    pub fn get_entities_by_mut(
        &mut self,
        component_type: ComponentType,
    ) -> Vec<&mut Entity> {
        let entities = self
            .entities
            .iter_mut()
            .filter(|e| match component_type {
                ComponentType::Position => {
                    if e.components.position.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::Velocity => {
                    if e.components.velocity.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::Size => {
                    if e.components.size.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::Render => {
                    if e.components.render.is_some() {
                        return true;
                    }
                    false
                }
                ComponentType::CameraFollow => {
                    if e.components.camera_follow.is_some() {
                        return true;
                    }
                    false
                }
            })
            .collect::<Vec<&mut Entity>>();
        entities
    }
}

pub mod collisions {
    use crate::ecs::{Entity, EntityId};
    use std::ops::Range;

    pub fn collision_pass(objects: &[Entity]) -> Vec<[EntityId; 2]> {
        let collisions = process_collisions(objects);
        collisions
    }

    #[derive(Debug, PartialEq)]
    struct CollisionBox {
        pub x: Range<u16>,
        pub y: Range<u16>,
    }

    impl CollisionBox {
        pub fn from_entity(entity: &Entity) -> Option<CollisionBox> {
            let Some(position) = &entity.components.position else {
                return None;
            };

            let Some(size) = &entity.components.size else {
                return None;
            };

            let Some(velocity) = &entity.components.velocity else {
                return None;
            };

            Some(CollisionBox {
                x: CollisionBox::range_from_physical_properties(
                    position.x,
                    size.x,
                    velocity.x,
                ),
                y: CollisionBox::range_from_physical_properties(
                    position.y,
                    size.y,
                    velocity.y,
                ),
            })
        }

        fn range_from_physical_properties(position: u16, size: u16, velocity: i16) -> Range<u16> {
            if velocity.is_positive() {
                return position..(position as i16 + size as i16 + velocity) as u16;
            }
            (position as i16 + velocity) as u16..position + size
        }
    }

    fn are_collision_boxes_overlapping(
        a_collision_box: &CollisionBox,
        b_collision_box: &CollisionBox,
    ) -> bool {
        fn overlapping_1d(a: &Range<u16>, b: &Range<u16>) -> bool {
            a.end > b.start && b.end > a.start
        }
        overlapping_1d(&a_collision_box.x, &b_collision_box.x)
            && overlapping_1d(&a_collision_box.y, &b_collision_box.y)
    }

    fn process_collisions(objects: &[Entity]) -> Vec<[EntityId; 2]> {
        // TODO: Collision is O(n^2) - worth looking at better approaches in future
        let mut collisions: Vec<[EntityId; 2]> = vec![];
        for i in 0..objects.len() {
            let (left, rest) = objects.split_at(i + 1);

            let a = &left[i];

            for b in rest {
                let Some(a_collision_box) = CollisionBox::from_entity(a) else {
                    continue;
                };
                let Some(b_collision_box) = CollisionBox::from_entity(b) else {
                    continue;
                };

                if are_collision_boxes_overlapping(&a_collision_box, &b_collision_box) {
                    collisions.push([a.id, b.id]);
                }
            }
        }
        collisions
    }

    #[cfg(test)]
    mod test {
        use crate::ecs::{
            collisions::{collision_pass, CollisionBox},
            Entity, EntityId,
        };

        #[test]
        fn test_collision_pass_static_same_place_entities() {
            let entity_1 = Entity::from_tuples(EntityId(0), (0, 0), (0, 0), (1, 1), None, false);
            let entity_2 = Entity::from_tuples(EntityId(1), (0, 0), (0, 0), (1, 1), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities);
            assert_eq!(1, collisions.len());
            assert_eq!(EntityId(0), collisions[0][0]);
            assert_eq!(EntityId(1), collisions[0][1]);
        }

        #[test]
        fn test_collision_pass_static_one_tile_gap_entities() {
            let entity_1 = Entity::from_tuples(EntityId(0), (0, 0), (0, 0), (1, 1), None, false);
            let entity_2 = Entity::from_tuples(EntityId(1), (2, 2), (0, 0), (1, 1), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities);
            assert_eq!(0, collisions.len());
        }

        #[test]
        fn test_collision_pass_static_adjacent_entities() {
            let entity_1 = Entity::from_tuples(EntityId(0), (0, 0), (0, 0), (1, 1), None, false);
            let entity_2 = Entity::from_tuples(EntityId(1), (1, 1), (0, 0), (1, 1), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities);
            assert_eq!(0, collisions.len());
        }

        #[test]
        fn test_collision_pass_crashing_entities() {
            let entity_1 = Entity::from_tuples(EntityId(0), (0, 0), (1, 1), (1, 1), None, false);
            let entity_2 = Entity::from_tuples(EntityId(1), (1, 1), (0, 0), (1, 1), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities);
            assert_eq!(1, collisions.len());
            assert_eq!(EntityId(0), collisions[0][0]);
            assert_eq!(EntityId(1), collisions[0][1]);
        }

        #[test]
        fn test_collision_pass_player_up_and_wall_should_collide() {
            let entity_player =
                Entity::from_tuples(EntityId(0), (5, 5), (0, 1), (2, 1), Some('O'), true);
            let entity_wall =
                Entity::from_tuples(EntityId(1), (5, 6), (0, 0), (2, 1), Some('#'), false);

            let entities = &[entity_player, entity_wall];
            let collisions = collision_pass(entities);
            assert_eq!(
                1,
                collisions.len(),
                "Expected a collision between player and wall"
            );
            let pair = collisions[0];
            assert!(
                (pair[0] == EntityId(0) && pair[1] == EntityId(1)),
                "Collision should be between EntityId(0) and EntityId(1), got: {:?}",
                pair
            );
        }

        #[test]
        fn test_collision_pass_player_down_and_wall_should_collide() {
            let entity_player =
                Entity::from_tuples(EntityId(0), (5, 6), (0, -1), (2, 1), Some('O'), true);
            let entity_wall =
                Entity::from_tuples(EntityId(1), (5, 5), (0, 0), (2, 1), Some('#'), false);

            let entities = &[entity_player, entity_wall];
            let collisions = collision_pass(entities);
            assert_eq!(
                1,
                collisions.len(),
                "Expected a collision between player and wall"
            );
            let pair = collisions[0];
            // The order of the pair may not be guaranteed, so check both possibilities
            assert!(
                (pair[0] == EntityId(0) && pair[1] == EntityId(1)),
                "Collision should be between EntityId(0) and EntityId(1), got: {:?}",
                pair
            );
        }

        #[test]
        fn test_entity_collision_box_moving() {
            let entity_player =
                Entity::from_tuples(EntityId(0), (5, 5), (0, -1), (2, 1), Some('O'), true);

            let maybe_collision_box = CollisionBox::from_entity(&entity_player);

            let collision_box = maybe_collision_box.expect("Collision box not created");

            assert_eq!(collision_box, CollisionBox { x: 5..7, y: 4..6 })
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    fn emptys() -> Components {
        Components {
            position: None,
            velocity: None,
            render: None,
            size: None,
            camera_follow: None,
        }
    }

    #[test]
    fn test_initialise_empty_ecs() {
        let ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0)
    }

    #[test]
    fn test_add_entity_with_nos() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        ecs.add_entity_froms(emptys());
        assert_eq!(ecs.entities.len(), 1);
    }

    #[test]
    fn test_get_entity_by_id() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        ecs.add_entity_froms(Components::new());
        assert_eq!(ecs.entities.len(), 1);

        let entity_from_ecs = ecs.get_entity_by_id(EntityId(0));
        assert_eq!(entity_from_ecs.unwrap().id, EntityId(0))
    }

    #[test]
    fn test_get_entities_by_ids() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity_one_id = ecs.add_entity_froms(Components {
            position: Some(PositionComponent { x: 0, y: 0 }),
            velocity: None,
            render: None,
            size: None,
            camera_follow: None,
        });
        let entity_two_id = ecs.add_entity_froms(Components {
            position: Some(PositionComponent { x: 1, y: 1 }),
            velocity: None,
            render: None,
            size: None,
            camera_follow: None,
        });
        assert_eq!(ecs.entities.len(), 2);

        let entity_one_from_ecs = ecs.get_entity_by_id(entity_one_id);
        assert_eq!(entity_one_from_ecs.unwrap().id, EntityId(0));
        let entity_position = &entity_one_from_ecs
            .unwrap()
            .components
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0);
        assert_eq!(entity_position.y, 0);

        let entity_two_from_ecs = ecs.get_entity_by_id(entity_two_id);
        assert_eq!(entity_two_from_ecs.unwrap().id, EntityId(1));
        let entity_position = &entity_two_from_ecs
            .unwrap()
            .components
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 1);
        assert_eq!(entity_position.y, 1);
    }

    #[test]
    fn test_ecs_step() {
        let mut ecs = ECS::new();
        assert_eq!(ecs.entities.len(), 0);
        let entity_one_id = ecs.add_entity_froms(Components {
            position: Some(PositionComponent { x: 0, y: 0 }),
            velocity: Some(VelocityComponent { x: 0, y: 0 }),
            render: None,
            size: None,
            camera_follow: None,
        });
        let entity_two_id = ecs.add_entity_froms(Components {
            position: Some(PositionComponent { x: 1, y: 1 }),
            velocity: Some(VelocityComponent { x: 1, y: 1 }),
            render: None,
            size: None,
            camera_follow: None,
        });
        assert_eq!(ecs.entities.len(), 2);

        let entity_from_ecs = ecs.get_entity_by_id(entity_one_id);
        assert_eq!(entity_from_ecs.unwrap().id, entity_one_id);
        let entity_velocity = &entity_from_ecs
            .unwrap()
            .components
            .velocity
            .as_ref()
            .unwrap();
        assert_eq!(entity_velocity.x, 0);
        assert_eq!(entity_velocity.y, 0);

        let entity_from_ecs = ecs.get_entity_by_id(entity_two_id);
        assert_eq!(entity_from_ecs.unwrap().id, entity_two_id);
        let entity_velocity = &entity_from_ecs
            .unwrap()
            .components
            .velocity
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
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0);
        assert_eq!(entity_position.y, 0);

        let entity_from_ecs = ecs.get_entity_by_id(entity_two_id);
        assert_eq!(entity_from_ecs.unwrap().id, entity_two_id);
        let entity_position = &entity_from_ecs
            .unwrap()
            .components
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 2);
        assert_eq!(entity_position.y, 2);
    }
}
