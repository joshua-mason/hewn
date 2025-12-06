use cgmath::{self, InnerSpace};
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
}

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
        }
    }
}

impl GameHandler for HelloGame {
    fn start_game(&mut self) {}

    fn next(&mut self, dt: Duration) {
        for mouse_event in self.game_controller.mouse_event_queue.drain(..).into_iter() {
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
                    let d_pos = cgmath::vec2(mouse_x - player_x, mouse_y - player_y);
                    let normalised_d_pos = d_pos.normalize();
                    player_entity.components.velocity.as_mut().unwrap().x = normalised_d_pos.x;
                    player_entity.components.velocity.as_mut().unwrap().y = normalised_d_pos.y;
                    let _ = player_entity;
                    self.ecs.add_entity_from_components(Components {
                        position: Some((mouse_x, mouse_y).into()),
                        velocity: None,
                        render: Some(RenderComponent {
                            ascii_character: '?',
                            rgb: (0.1, 0.1, 0.1).into(),
                        }),
                        size: Some(SizeComponent { x: 0.1, y: 0.1 }),
                        camera_follow: None,
                    });
                }
                MouseEvent::CursorMoved(location) => {
                    self.game_controller.mouse_location = location;
                }
                _ => {}
            }
        }

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

pub mod path_finding {
    //! Path-finding utilities using the A* search algorithm.
    //!
    //! This module provides helper functions for grid-based pathfinding, using the
    //! A* algorithm to find shortest paths while supporting obstacles and grid bounds.
    //!
    //! The A* algorithm reference: https://en.wikipedia.org/wiki/A*_search_algorithm

    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashMap, HashSet};

    pub type Node = (isize, isize);

    #[derive(Clone, Eq, PartialEq)]
    struct State {
        f_score: u32, // g_score + h_score (f_score)
        position: Node,
    }

    // Implement Ord for State to make BinaryHeap a min-heap on f_score.
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .f_score
                .cmp(&self.f_score)
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn a_star_path(
        start: Node,
        end: Node,
        blocked_nodes: &HashSet<Node>,
        bounds: (isize, isize), // (width, height) assuming 0,0 bottom-left
    ) -> Option<Vec<Node>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<Node, Node> = HashMap::new();
        let mut g_score: HashMap<Node, u32> = HashMap::new();

        g_score.insert(start, 0);
        open_set.push(State {
            f_score: heuristic(start, end),
            position: start,
        });

        while let Some(State {
            f_score: _,
            position,
        }) = open_set.pop()
        {
            if position == end {
                return Some(reconstruct_path(came_from, end));
            }

            for (neighbor, move_cost) in get_neighbors(position, bounds) {
                if blocked_nodes.contains(&neighbor) {
                    continue;
                }

                // Corner Cutting Check:
                // If moving diagonally, check if the two adjacent cardinals are blocked.
                if move_cost == 14 {
                    let dx = neighbor.0 - position.0;
                    let dy = neighbor.1 - position.1;
                    if blocked_nodes.contains(&(position.0 + dx, position.1))
                        || blocked_nodes.contains(&(position.0, position.1 + dy))
                    {
                        continue;
                    }
                }

                let current_g_score = *g_score
                    .get(&position)
                    .expect("Node in open_set must be in g_score");
                let tentative_g_score = current_g_score + move_cost;

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, position);
                    g_score.insert(neighbor, tentative_g_score);
                    open_set.push(State {
                        f_score: tentative_g_score + heuristic(neighbor, end),
                        position: neighbor,
                    });
                }
            }
        }

        None
    }

    pub fn world_to_grid(
        x: f32,
        y: f32,
        grid_origin: (f32, f32),
        cell_size: f32,
    ) -> (isize, isize) {
        let gx = ((x - grid_origin.0) / cell_size).floor() as isize;
        let gy = ((y - grid_origin.1) / cell_size).floor() as isize;
        (gx, gy)
    }

    pub fn grid_to_world(
        gx: isize,
        gy: isize,
        grid_origin: (f32, f32),
        cell_size: f32,
    ) -> (f32, f32) {
        (
            grid_origin.0 + (gx as f32 * cell_size) + (cell_size / 2.0),
            grid_origin.1 + (gy as f32 * cell_size) + (cell_size / 2.0),
        )
    }

    fn heuristic(a: Node, b: Node) -> u32 {
        // Octile Distance
        // cost = 10 * (dx + dy) + (14 - 2 * 10) * min(dx, dy)
        //      = 10 * (dx + dy) - 6 * min(dx, dy)
        let dx = (a.0 - b.0).abs() as u32;
        let dy = (a.1 - b.1).abs() as u32;
        if dx > dy {
            14 * dy + 10 * (dx - dy)
        } else {
            14 * dx + 10 * (dy - dx)
        }
    }

    fn get_neighbors(node: Node, bounds: (isize, isize)) -> Vec<(Node, u32)> {
        let (x, y) = node;
        let (w, h) = bounds;
        let mut neighbors = Vec::new();

        // (dx, dy, cost)
        let dirs = [
            // Cardinals (Cost 10)
            (0, 1, 10),
            (0, -1, 10),
            (1, 0, 10),
            (-1, 0, 10),
            // Diagonals (Cost 14)
            (1, 1, 14),
            (1, -1, 14),
            (-1, 1, 14),
            (-1, -1, 14),
        ];

        for (dx, dy, f_score) in dirs {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < w && ny >= 0 && ny < h {
                neighbors.push(((nx, ny), f_score));
            }
        }

        neighbors
    }

    fn reconstruct_path(came_from: HashMap<Node, Node>, current: Node) -> Vec<Node> {
        let mut total_path = vec![current];
        let mut curr = current;
        while let Some(&prev) = came_from.get(&curr) {
            total_path.push(prev);
            curr = prev;
        }
        total_path.reverse();
        total_path
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn test_simple_path() {
            let start = (0, 0);
            let end = (2, 0);
            let blocked = HashSet::new();
            let bounds = (10, 10);

            let path = a_star_path(start, end, &blocked, bounds).expect("No path found");
            assert_eq!(path, vec![(0, 0), (1, 0), (2, 0)]);
        }

        #[test]
        fn test_blocked_path() {
            let start = (0, 0);
            let end = (0, 2);
            // Block (0,1) so it has to go around
            let mut blocked = HashSet::new();
            blocked.insert((0, 1));
            let bounds = (10, 10);

            let path = a_star_path(start, end, &blocked, bounds).expect("No path found");
            // Path should be (0,0) -> (1,0) -> (1,1) -> (1,2) -> (0,2) OR similar equal cost path
            // Just check length is 5 (0,0, 1,0, 1,1, 1,2, 0,2) or (0,0, 1,0, 1,1, 0,1-BLOCKED)
            // actually manhattan distance around one block:
            // 0,0 -> 1,0 -> 1,1 -> 0,1 (blocked)
            // 0,0 -> 1,0 -> 1,1 -> 0,1 x
            // 0,0 -> 1,0 -> 1,1 -> 1,2 -> 0,2 = 5 steps
            assert_eq!(path.len(), 5);
            assert_eq!(path.first(), Some(&start));
            assert_eq!(path.last(), Some(&end));
        }

        #[test]
        fn test_no_path() {
            let start = (0, 0);
            let end = (5, 5);
            let mut blocked = HashSet::new();
            // Wall off the start
            blocked.insert((0, 1));
            blocked.insert((1, 0));
            let bounds = (10, 10);

            let path = a_star_path(start, end, &blocked, bounds);
            assert!(path.is_none());
        }

        #[test]
        fn test_u_shape_obstacle() {
            // This test creates a vertical wall obstacle between the start and end points.
            // Since the direct path (y=2) and the path below (y=0, y=1) are blocked,
            // the algorithm is forced to find a path that goes 'over' the wall (y=3).
            //
            // Layout:
            // y=3  . . .  (Open path)
            // y=2  S | E  (Wall at x=1)
            // y=1  . | .  (Wall at x=1)
            // y=0  . | .  (Wall at x=1)
            //      0 1 2

            let start = (0, 2);
            let end = (2, 2);
            let bounds = (5, 5);

            let mut blocked = HashSet::new();
            blocked.insert((1, 0));
            blocked.insert((1, 1));
            blocked.insert((1, 2)); // Block direct path at y=2

            let path = a_star_path(start, end, &blocked, bounds).expect("Path found");

            // Verify the path went over the wall (y > 2) at x=1
            let crossed_over = path.iter().any(|n| n.0 == 1 && n.1 > 2);
            assert!(crossed_over, "Path should cross over the wall at y > 2");
        }

        #[test]
        fn test_zig_zag_maze() {
            // This test creates a maze that forces the pathfinder to zig-zag
            // down and then back up to reach the destination.
            //
            // Map Layout (5x5):
            // y=4  . . . . .
            // y=3  x x x x .  (Top wall blocking y=3, x=0..3)
            // y=2  S . x . E  (Start at 0,2. Block at 3,2)
            // y=1  x x . x .  (Walls at x=0,1 and x=3. Gaps at x=2 and x=4)
            // y=0  . . . . .  (Bottom passage)
            //      0 1 2 3 4

            let start = (0, 2);
            let end = (4, 2);
            let bounds = (5, 5);

            let mut blocked = HashSet::new();

            // Top wall: blocks (0,3) to (3,3)
            for x in 0..4 {
                blocked.insert((x, 3));
            }

            // Left-bottom wall: blocks (0,1) and (1,1)
            for x in 0..2 {
                blocked.insert((x, 1));
            }

            // Right-bottom wall: blocks (3,1). Note: (4,1) is OPEN.
            blocked.insert((3, 1));

            // Center block: blocks direct path at (3,2)
            blocked.insert((3, 2));

            // Expected Path Route:
            // 1. Start (0,2) -> Right to (2,2)
            // 2. Down through gap at (2,1)
            // 3. Down to bottom passage (2,0) -> Right to (4,0)
            // 4. Up through gap at (4,1) -> Reach End (4,2)

            let path = a_star_path(start, end, &blocked, bounds).expect("Path found");

            assert!(path.contains(&(2, 2)), "Should go right from start");
            assert!(path.contains(&(2, 1)), "Should go down through first gap");
            assert!(path.contains(&(2, 0)), "Should reach bottom passage");
            assert!(path.contains(&(4, 1)), "Should go up through second gap");
            assert_eq!(path.first(), Some(&start));
            assert_eq!(path.last(), Some(&end));
        }

        #[test]
        fn test_large_open_field() {
            let start = (0, 0);
            let end = (10, 10);
            let blocked = HashSet::new();
            let bounds = (20, 20);

            let path = a_star_path(start, end, &blocked, bounds).expect("Path found");

            // With 8-way movement, the shortest path is a straight diagonal line.
            // (0,0) -> (1,1) -> ... -> (10,10)
            // Length = 10 steps + 1 start node = 11 nodes.
            assert_eq!(path.len(), 11);
        }
        #[test]
        fn test_coordinate_conversion() {
            let origin = (0.0, 0.0);
            let cell_size = 0.5;

            // Test 0,0
            assert_eq!(world_to_grid(0.0, 0.0, origin, cell_size), (0, 0));
            assert_eq!(world_to_grid(0.49, 0.49, origin, cell_size), (0, 0));

            // Test boundary
            assert_eq!(world_to_grid(0.5, 0.5, origin, cell_size), (1, 1));

            // Test Grid to World (centers)
            let (wx, wy) = grid_to_world(0, 0, origin, cell_size);
            assert_eq!(wx, 0.25);
            assert_eq!(wy, 0.25);

            let (wx, wy) = grid_to_world(1, 1, origin, cell_size);
            assert_eq!(wx, 0.75);
            assert_eq!(wy, 0.75);
        }

        #[test]
        fn test_path_to_blocked_target() {
            let start = (0, 0);
            let end = (2, 0);
            let mut blocked = HashSet::new();
            blocked.insert(end); // The target itself is blocked
            let bounds = (10, 10);

            // Should fail because we can't enter the blocked node
            let path = a_star_path(start, end, &blocked, bounds);
            assert!(path.is_none());
        }

        #[test]
        fn test_path_from_blocked_start() {
            let start = (0, 0);
            let end = (2, 0);
            let mut blocked = HashSet::new();
            blocked.insert(start); // The start itself is blocked
            let bounds = (10, 10);

            // Should find a path?
            // Logic: Start is added to open_set manually.
            // Neighbors are checked.
            // As long as neighbors aren't blocked, it should proceed.
            let path = a_star_path(start, end, &blocked, bounds)
                .expect("Should find path even if start is technically marked blocked");
            assert_eq!(path.last(), Some(&end));
        }
    }
}
