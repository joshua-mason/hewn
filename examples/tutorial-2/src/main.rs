use cgmath::{self};
use hewn::ecs::{Components, EntityId, PositionComponent, RenderComponent, SizeComponent};
use hewn::runtime::{MouseEvent, MouseLocation, RuntimeEvent};
use hewn::wgpu::runtime::WindowRuntime;
use hewn::{ecs::ECS, runtime::GameHandler};
use hewn::{ecs::VelocityComponent, runtime::Key};
use std::time::Duration;

pub struct GameController {
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    mouse_event_queue: Vec<MouseEvent>,
    mouse_location: MouseLocation,
}

impl GameController {
    pub(crate) fn new() -> Self {
        Self {
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            mouse_event_queue: Vec::new(),
            mouse_location: MouseLocation { x: 0.0, y: 0.0 },
        }
    }

    pub(crate) fn handle_key(&mut self, key: Key, is_pressed: bool) -> bool {
        match key {
            Key::Up => {
                self.is_up_pressed = is_pressed;
                true
            }
            Key::Down => {
                self.is_down_pressed = is_pressed;
                true
            }
            Key::Left => {
                self.is_left_pressed = is_pressed;
                true
            }
            Key::Right => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent) -> bool {
        self.mouse_event_queue.push(mouse);
        true
    }
}
struct HelloGame {
    ecs: ECS,
    player_id: EntityId,
    game_controller: GameController,
    current_path: Option<Vec<(f32, f32)>>, // Added path storage
}

const CELL_SIZE: f32 = 1.25;
const GRID_ORIGIN: (f32, f32) = (0.0, 0.0);
const GRID_BOUNDS: (isize, isize) = (100, 100); // 50.0 width/height at 0.5 cell size

impl HelloGame {
    fn new() -> Self {
        let mut ecs = ECS::new();

        let player_id = ecs.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 5.0, y: 5.0 }),
            render: Some(RenderComponent {
                ascii_character: '@',
                rgb: cgmath::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            }),
            velocity: Some(VelocityComponent { x: 0.0, y: 0.0 }),
            size: Some(SizeComponent { x: 1.0, y: 1.0 }),
            camera_follow: None,
        });
        ecs.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 8.0, y: 5.0 }),
            render: Some(RenderComponent {
                ascii_character: '#',
                rgb: cgmath::Vector3 {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                },
            }),
            velocity: None,
            size: Some(SizeComponent { x: 2.0, y: 1.0 }),
            camera_follow: None,
        });

        Self {
            ecs,
            player_id,
            game_controller: GameController::new(),
            current_path: None,
        }
    }

    fn get_blocked_nodes(&self) -> std::collections::HashSet<(isize, isize)> {
        let mut blocked_nodes = std::collections::HashSet::new();
        let entities = self
            .ecs
            .get_entities_with_component(hewn::ecs::ComponentType::Size);
        for entity in entities {
            if entity.id == self.player_id {
                continue;
            }
            if let (Some(pos), Some(size)) = (entity.components.position, entity.components.size) {
                let min_x =
                    hewn::pathfinding::world_to_grid(pos.x, pos.y, GRID_ORIGIN, CELL_SIZE).0;
                let max_x = hewn::pathfinding::world_to_grid(
                    pos.x + size.x - 0.01,
                    pos.y,
                    GRID_ORIGIN,
                    CELL_SIZE,
                )
                .0;
                let min_y =
                    hewn::pathfinding::world_to_grid(pos.x, pos.y, GRID_ORIGIN, CELL_SIZE).1;
                let max_y = hewn::pathfinding::world_to_grid(
                    pos.x,
                    pos.y + size.y - 0.01,
                    GRID_ORIGIN,
                    CELL_SIZE,
                )
                .1;

                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        blocked_nodes.insert((x, y));
                    }
                }
            }
        }
        blocked_nodes
    }

    fn compute_path_to_target(
        &self,
        start: (f32, f32),
        end: (f32, f32),
    ) -> Option<Vec<(f32, f32)>> {
        let start_node = hewn::pathfinding::world_to_grid(start.0, start.1, GRID_ORIGIN, CELL_SIZE);
        let end_node = hewn::pathfinding::world_to_grid(end.0, end.1, GRID_ORIGIN, CELL_SIZE);

        // Determine agent size in grid cells.
        // Player is 1.0x1.0. Cell size is 0.5. So agent is 2x2 cells.
        // Ideally, we'd fetch this from the player entity components.
        let agent_size_cells = (
            (1.0 / CELL_SIZE).ceil() as usize,
            (1.0 / CELL_SIZE).ceil() as usize,
        );

        let raw_blocked_nodes = self.get_blocked_nodes();
        let inflated_obstacles =
            hewn::pathfinding::inflate_obstacles(&raw_blocked_nodes, agent_size_cells);

        hewn::pathfinding::a_star_path(start_node, end_node, &inflated_obstacles, GRID_BOUNDS).map(
            |path| {
                path.iter()
                    .map(|node| {
                        hewn::pathfinding::grid_to_world(node.0, node.1, GRID_ORIGIN, CELL_SIZE)
                    })
                    .collect()
            },
        )
    }

    fn move_player_along_path(&mut self) {
        if let Some(path) = &mut self.current_path {
            if let Some(target) = path.first() {
                let player_entity = self
                    .ecs
                    .get_entity_by_id_mut(self.player_id)
                    .expect("Player must exist");
                let pos = player_entity
                    .components
                    .position
                    .as_mut()
                    .expect("Player must have position");
                let velocity = player_entity
                    .components
                    .velocity
                    .as_mut()
                    .expect("Player must have velocity");

                let dx = target.0 - pos.x;
                let dy = target.1 - pos.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < 0.01 {
                    path.remove(0);
                    if path.is_empty() {
                        self.current_path = None;
                        velocity.x = 0.0;
                        velocity.y = 0.0;
                    }
                } else {
                    let speed = 5.0;
                    let dist = dist_sq.sqrt();
                    velocity.x = (dx / dist) * speed;
                    velocity.y = (dy / dist) * speed;
                }
            }
        }
    }
}

impl GameHandler for HelloGame {
    fn start_game(&mut self) {}

    fn next(&mut self, dt: Duration) {
        let events: Vec<MouseEvent> = self.game_controller.mouse_event_queue.drain(..).collect();
        for mouse_event in events {
            match mouse_event {
                MouseEvent::LeftClick => {
                    let player_entity = self.ecs.get_entity_by_id_mut(self.player_id);
                    let Some(player_entity) = player_entity else {
                        return;
                    };

                    let mouse_x = self.game_controller.mouse_location.x;
                    let player_x = player_entity.components.position.as_mut().unwrap().x;
                    let mouse_y = self.game_controller.mouse_location.y;
                    let player_y = player_entity.components.position.as_mut().unwrap().y;

                    if let Some(path) =
                        self.compute_path_to_target((player_x, player_y), (mouse_x, mouse_y))
                    {
                        self.current_path = Some(path);
                        println!(
                            "Path found! Length: {}",
                            self.current_path.as_ref().unwrap().len()
                        );
                    } else {
                        println!("No path found!");
                        self.current_path = None;
                    }

                    self.ecs.add_entity_from_components(Components {
                        position: Some((mouse_x, mouse_y).into()),
                        velocity: None,
                        render: None,
                        size: None,
                        camera_follow: None,
                    });
                }
                MouseEvent::CursorMoved(location) => {
                    self.game_controller.mouse_location = location;
                }
                _ => {}
            }
        }

        self.move_player_along_path();

        let collisions = self.ecs.collision_pass(dt);
        for [a, b] in collisions.into_iter() {
            if a == self.player_id || b == self.player_id {
                let player_entity = self.ecs.get_entity_by_id_mut(self.player_id);
                let Some(player_entity) = player_entity else {
                    return;
                };
                let Some(velocity) = &mut player_entity.components.velocity else {
                    return;
                };
                velocity.x = 0.0;
                velocity.y = 0.0;
                break;
            }
        }

        self.ecs.step(dt);
    }

    fn handle_event(&mut self, event: RuntimeEvent) -> bool {
        match event {
            RuntimeEvent::Key(key_event) => self
                .game_controller
                .handle_key(key_event.key, key_event.pressed),
            RuntimeEvent::Mouse(mouse_event) => self.game_controller.handle_mouse(mouse_event),
        }
    }

    fn ecs(&self) -> &ECS {
        &self.ecs
    }

    fn debug_str(&self) -> Option<String> {
        let player = self.ecs.get_entity_by_id(self.player_id)?;
        let pos = player.components.position.as_ref()?;
        Some(format!("Player @ ({}, {})", pos.x, pos.y))
    }
}

fn main() {
    let mut game = HelloGame::new();
    let mut runtime = WindowRuntime::new();
    let entity_id = game.player_id;
    let _ = runtime.start(
        &mut game,
        hewn::wgpu::render::CameraStrategy::CameraFollow(entity_id),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use hewn::runtime::MouseEvent;
    use hewn::runtime::MouseLocation;

    #[test]
    fn test_game_pathfinding_around_block() {
        let mut game = HelloGame::new();

        // HelloGame::new() spawns:
        // Player @ 5.0, 5.0
        // Block @ 8.0, 5.0 (Size 2.0, 1.0)

        // Simulate Click beyond the block
        // Block is x=[8, 10], y=[5, 6].
        // Target: 12.0, 5.0.

        // 1. Set cursor position
        game.game_controller.mouse_location = MouseLocation { x: 12.0, y: 5.0 };
        // 2. Push Left Click
        game.game_controller
            .mouse_event_queue
            .push(MouseEvent::LeftClick);

        // 3. Run one frame
        game.next(Duration::from_millis(16));

        // 4. Check if path was found
        assert!(
            game.current_path.is_some(),
            "Path should be found around the block"
        );

        let path = game.current_path.as_ref().unwrap();
        assert!(!path.is_empty());

        // Verify the last point is roughly near target
        let last = path.last().unwrap();
        assert!((last.0 - 12.0).abs() < 1.0);
        assert!((last.1 - 5.0).abs() < 1.0);

        // Verify it doesn't go THROUGH the block
        // Blocked Grid X: [16, 19], Y: [10, 11] (Cell 0.5)
        // World X: [8.0, 10.0], Y: [5.0, 6.0]

        for point in path {
            let in_block_x = point.0 >= 8.0 && point.0 <= 10.0;
            let in_block_y = point.1 >= 5.0 && point.1 <= 6.0;
            assert!(
                !(in_block_x && in_block_y),
                "Path point {:?} is inside the block!",
                point
            );
        }
    }
}
