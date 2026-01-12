use std::time::Duration;

use cgmath::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub id: EntityId,
    pub components: Components,
}

#[derive(Debug, Clone, Copy, Default)]
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

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy, Default)]
pub struct EntityId(pub u16);

impl Entity {
    pub fn new(id: EntityId) -> Entity {
        Entity {
            id,
            components: Components::new(),
        }
    }

    pub fn from_tuples(
        id: EntityId,
        pos: (f32, f32),
        vel: (f32, f32),
        size: (f32, f32),
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
                render: ascii_character.map(|c| RenderComponent {
                    ascii_character: c,
                    rgb: Vector3::new(0.0, 0.0, 0.0),
                    sprite_tile: None,
                }),
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
#[derive(Debug, Clone, Copy)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

impl Component for PositionComponent {
    const TYPE: ComponentType = ComponentType::Position;
}

impl From<(f32, f32)> for PositionComponent {
    fn from(tuple: (f32, f32)) -> Self {
        PositionComponent {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VelocityComponent {
    pub x: f32,
    pub y: f32,
}
impl Component for VelocityComponent {
    const TYPE: ComponentType = ComponentType::Velocity;
}

#[derive(Debug, Clone, Copy)]
pub struct SizeComponent {
    pub x: f32,
    pub y: f32,
}
impl Component for SizeComponent {
    const TYPE: ComponentType = ComponentType::Size;
}

/// Component that controls how an entity is rendered.
///
/// - `ascii_character`: The character to display in terminal mode
/// - `rgb`: The color (used for solid color rendering or tinting)
/// - `sprite_tile`: Optional tile index in the tilemap (0-indexed, row-major order)
///                  If None, renders as solid color. If Some(n), uses tile n from tilemap.
#[derive(Debug, Clone, Copy)]
pub struct RenderComponent {
    pub ascii_character: char,
    pub rgb: Vector3<f32>,
    pub sprite_tile: Option<u16>,
}
impl Component for RenderComponent {
    const TYPE: ComponentType = ComponentType::Render;
}

#[derive(Debug, Clone, Copy)]
pub struct CameraFollow {}
impl Component for CameraFollow {
    const TYPE: ComponentType = ComponentType::CameraFollow;
}

#[derive(Default)]
pub struct Scene {
    next_entity_id: EntityId,
    entities: Vec<Entity>,
}

impl Scene {
    pub fn step(&mut self, dt: Duration) {
        // Consider splitting systems e.g. if we are handling gravity in the future
        let velocities = self.get_entities_by_mut(ComponentType::Velocity);
        for c in velocities {
            let Some(position) = &mut c.components.position else {
                continue;
            };

            let Some(velocity) = &mut c.components.velocity else {
                continue;
            };

            if velocity.x != 0.0 {
                position.x = position.x + velocity.x * dt.as_secs_f32();
            }
            if velocity.y != 0.0 {
                position.y = position.y + velocity.y * dt.as_secs_f32();
            }
        }
    }

    pub fn collision_pass(&self, dt: Duration) -> Vec<[EntityId; 2]> {
        collisions::collision_pass(&self.entities, dt)
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
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

    pub fn get_entities_with_component(&self, component_type: ComponentType) -> Vec<&Entity> {
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

    pub fn get_entities_by_mut(&mut self, component_type: ComponentType) -> Vec<&mut Entity> {
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
    use crate::scene::{Entity, EntityId, VelocityComponent};
    use std::{ops::Range, time::Duration};

    #[derive(Debug, PartialEq)]
    struct CollisionBox {
        pub x: Range<f32>,
        pub y: Range<f32>,
    }

    impl CollisionBox {
        pub fn from_entity(entity: &Entity, dt: Duration) -> Option<CollisionBox> {
            let Some(position) = &entity.components.position else {
                return None;
            };

            let Some(size) = &entity.components.size else {
                return None;
            };

            let velocity = entity
                .components
                .velocity
                .as_ref()
                .unwrap_or(&VelocityComponent { x: 0.0, y: 0.0 });

            Some(CollisionBox {
                x: CollisionBox::range_from_physical_properties(
                    position.x,
                    size.x,
                    velocity.x * dt.as_secs_f32(),
                ),
                y: CollisionBox::range_from_physical_properties(
                    position.y,
                    size.y,
                    velocity.y * dt.as_secs_f32(),
                ),
            })
        }

        fn range_from_physical_properties(position: f32, size: f32, velocity: f32) -> Range<f32> {
            if velocity.is_sign_positive() {
                return position..(position + size + velocity);
            }
            (position + velocity)..position + size
        }
    }

    fn are_collision_boxes_overlapping(
        a_collision_box: &CollisionBox,
        b_collision_box: &CollisionBox,
    ) -> bool {
        fn overlapping_1d(a: &Range<f32>, b: &Range<f32>) -> bool {
            a.end > b.start && b.end > a.start
        }
        overlapping_1d(&a_collision_box.x, &b_collision_box.x)
            && overlapping_1d(&a_collision_box.y, &b_collision_box.y)
    }

    pub fn collision_pass(objects: &[Entity], dt: Duration) -> Vec<[EntityId; 2]> {
        // TODO: Collision is O(n^2) - worth looking at better approaches in future
        let mut collisions: Vec<[EntityId; 2]> = vec![];
        for i in 0..objects.len() {
            let (left, rest) = objects.split_at(i + 1);

            let a = &left[i];

            for b in rest {
                let Some(a_collision_box) = CollisionBox::from_entity(a, dt) else {
                    continue;
                };
                let Some(b_collision_box) = CollisionBox::from_entity(b, dt) else {
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
        use std::time::Duration;

        use crate::scene::{
            collisions::{collision_pass, CollisionBox},
            Entity, EntityId,
        };

        #[test]
        fn test_collision_pass_static_same_place_entities() {
            let entity_1 =
                Entity::from_tuples(EntityId(0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0), None, false);
            let entity_2 =
                Entity::from_tuples(EntityId(1), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities, Duration::from_secs(1));
            assert_eq!(1, collisions.len());
            assert_eq!(EntityId(0), collisions[0][0]);
            assert_eq!(EntityId(1), collisions[0][1]);
        }

        #[test]
        fn test_collision_pass_static_one_tile_gap_entities() {
            let entity_1 =
                Entity::from_tuples(EntityId(0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0), None, false);
            let entity_2 =
                Entity::from_tuples(EntityId(1), (2.0, 2.0), (0.0, 0.0), (1.0, 1.0), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities, Duration::from_secs(1));
            assert_eq!(0, collisions.len());
        }

        #[test]
        fn test_collision_pass_static_adjacent_entities() {
            let entity_1 =
                Entity::from_tuples(EntityId(0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0), None, false);
            let entity_2 =
                Entity::from_tuples(EntityId(1), (1.0, 1.0), (0.0, 0.0), (1.0, 1.0), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities, Duration::from_secs(1));
            assert_eq!(0, collisions.len());
        }

        #[test]
        fn test_collision_pass_crashing_entities() {
            let entity_1 =
                Entity::from_tuples(EntityId(0), (0.0, 0.0), (1.0, 1.0), (1.0, 1.0), None, false);
            let entity_2 =
                Entity::from_tuples(EntityId(1), (1.0, 1.0), (0.0, 0.0), (1.0, 1.0), None, false);

            let entities = &[entity_1, entity_2];
            let collisions = collision_pass(entities, Duration::from_secs(1));
            assert_eq!(1, collisions.len());
            assert_eq!(EntityId(0), collisions[0][0]);
            assert_eq!(EntityId(1), collisions[0][1]);
        }

        #[test]
        fn test_collision_pass_player_up_and_wall_should_collide() {
            let entity_player = Entity::from_tuples(
                EntityId(0),
                (5.0, 5.0),
                (0.0, 1.0),
                (2.0, 1.0),
                Some('O'),
                true,
            );
            let entity_wall = Entity::from_tuples(
                EntityId(1),
                (5.0, 6.0),
                (0.0, 0.0),
                (2.0, 1.0),
                Some('#'),
                false,
            );

            let entities = &[entity_player, entity_wall];
            let collisions = collision_pass(entities, Duration::from_secs(1));
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
            let entity_player = Entity::from_tuples(
                EntityId(0),
                (5.0, 6.0),
                (0.0, -1.0),
                (2.0, 1.0),
                Some('O'),
                true,
            );
            let entity_wall = Entity::from_tuples(
                EntityId(1),
                (5.0, 5.0),
                (0.0, 0.0),
                (2.0, 1.0),
                Some('#'),
                false,
            );

            let entities = &[entity_player, entity_wall];
            let collisions = collision_pass(entities, Duration::from_secs(1));
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
            let entity_player = Entity::from_tuples(
                EntityId(0),
                (5.0, 5.0),
                (0.0, -1.0),
                (2.0, 1.0),
                Some('O'),
                true,
            );

            let maybe_collision_box =
                CollisionBox::from_entity(&entity_player, Duration::from_secs(1));

            let collision_box = maybe_collision_box.expect("Collision box not created");

            assert_eq!(
                collision_box,
                CollisionBox {
                    x: 5.0..7.0,
                    y: 4.0..6.0
                }
            )
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
    fn test_initialise_empty_scene() {
        let scene = Scene::new();
        assert_eq!(scene.entities.len(), 0)
    }

    #[test]
    fn test_add_entity_with_nos() {
        let mut scene = Scene::new();
        assert_eq!(scene.entities.len(), 0);
        scene.add_entity_from_components(emptys());
        assert_eq!(scene.entities.len(), 1);
    }

    #[test]
    fn test_get_entity_by_id() {
        let mut scene = Scene::new();
        assert_eq!(scene.entities.len(), 0);
        scene.add_entity_from_components(Components::new());
        assert_eq!(scene.entities.len(), 1);

        let entity_from_scene = scene.get_entity_by_id(EntityId(0));
        assert_eq!(entity_from_scene.unwrap().id, EntityId(0))
    }

    #[test]
    fn test_get_entities_by_ids() {
        let mut scene = Scene::new();
        assert_eq!(scene.entities.len(), 0);
        let entity_one_id = scene.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 0.0, y: 0.0 }),
            velocity: None,
            render: None,
            size: None,
            camera_follow: None,
        });
        let entity_two_id = scene.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 1.0, y: 1.0 }),
            velocity: None,
            render: None,
            size: None,
            camera_follow: None,
        });
        assert_eq!(scene.entities.len(), 2);

        let entity_one_from_scene = scene.get_entity_by_id(entity_one_id);
        assert_eq!(entity_one_from_scene.unwrap().id, EntityId(0));
        let entity_position = &entity_one_from_scene
            .unwrap()
            .components
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0.0);
        assert_eq!(entity_position.y, 0.0);

        let entity_two_from_scene = scene.get_entity_by_id(entity_two_id);
        assert_eq!(entity_two_from_scene.unwrap().id, EntityId(1));
        let entity_position = &entity_two_from_scene
            .unwrap()
            .components
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 1.0);
        assert_eq!(entity_position.y, 1.0);
    }

    #[test]
    fn test_scene_step() {
        let mut scene = Scene::new();
        assert_eq!(scene.entities.len(), 0);
        let entity_one_id = scene.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 0.0, y: 0.0 }),
            velocity: Some(VelocityComponent { x: 0.0, y: 0.0 }),
            render: None,
            size: None,
            camera_follow: None,
        });
        let entity_two_id = scene.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 1.0, y: 1.0 }),
            velocity: Some(VelocityComponent { x: 1.0, y: 1.0 }),
            render: None,
            size: None,
            camera_follow: None,
        });
        assert_eq!(scene.entities.len(), 2);

        let entity_from_scene = scene.get_entity_by_id(entity_one_id);
        assert_eq!(entity_from_scene.unwrap().id, entity_one_id);
        let entity_velocity = &entity_from_scene
            .unwrap()
            .components
            .velocity
            .as_ref()
            .unwrap();
        assert_eq!(entity_velocity.x, 0.0);
        assert_eq!(entity_velocity.y, 0.0);

        let entity_from_scene = scene.get_entity_by_id(entity_two_id);
        assert_eq!(entity_from_scene.unwrap().id, entity_two_id);
        let entity_velocity = &entity_from_scene
            .unwrap()
            .components
            .velocity
            .as_ref()
            .unwrap();
        assert_eq!(entity_velocity.x, 1.0);
        assert_eq!(entity_velocity.y, 1.0);

        scene.step(Duration::from_secs(1));

        let entity_from_scene = scene.get_entity_by_id(entity_one_id);
        assert_eq!(entity_from_scene.unwrap().id, entity_one_id);
        let entity_position = &entity_from_scene
            .unwrap()
            .components
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 0.0);
        assert_eq!(entity_position.y, 0.0);

        let entity_from_scene = scene.get_entity_by_id(entity_two_id);
        assert_eq!(entity_from_scene.unwrap().id, entity_two_id);
        let entity_position = &entity_from_scene
            .unwrap()
            .components
            .position
            .as_ref()
            .unwrap();
        assert_eq!(entity_position.x, 2.0);
        assert_eq!(entity_position.y, 2.0);
    }
}
