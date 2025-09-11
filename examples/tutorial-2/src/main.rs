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
    /// All coordinates x going positive from left to right, and y positive going bottom to top
    use std::{cmp::Ordering, collections::HashMap};

    type Node = (usize, usize);
    type NodePath = Vec<Node>;
    type Nodes = Vec<Node>;
    type NodeTree = HashMap<Node, Nodes>; //HashMap<Node, HashMap<Node>>;

    pub fn a_star_path(
        node_tree: NodeTree,
        start_node: Node,
        end_node: Node,
    ) -> Result<NodePath, String> {
        let mut node_scores: HashMap<Node, f32> = HashMap::new(); // value is score
        let mut current_node = start_node;
        let mut node_history: Vec<Node> = vec![];
        let mut iterations = 0;
        let mut evaluated_nodes: Vec<Node> = vec![];
        let max_iter = 20;
        while iterations < max_iter {
            iterations += 1;
            let next_options = node_tree.get(&current_node).ok_or("Node not found")?;
            println!("current_node: {:?}", current_node);
            println!("next_options: {:?}", next_options);
            let mut new_node_scores = next_options
                .iter()
                .filter(|n| !evaluated_nodes.contains(n))
                .map(|n| {
                    let node_distance = 1; // TODO get distance from the current node we are considering. or is this frmo the starting node?
                    let euclidean_distance = (f32::powf(end_node.0 as f32 - n.0 as f32, 2.0)
                        + f32::powf(end_node.1 as f32 - n.1 as f32, 2.0))
                    .sqrt();
                    (n.clone(), euclidean_distance + node_distance as f32)
                })
                .collect::<HashMap<Node, f32>>();

            for node_score_entry in new_node_scores.iter_mut() {
                let existing_entry = node_scores
                    .entry(*node_score_entry.0)
                    .or_insert(node_score_entry.1.clone());
                if existing_entry > node_score_entry.1 {
                    *existing_entry = *node_score_entry.1
                }
            }

            node_history.push(current_node);
            evaluated_nodes.push(current_node.clone());
            node_scores.remove(&current_node);
            current_node = node_scores
                .iter()
                .min_by(|x, y| x.1.partial_cmp(y.1).unwrap_or(Ordering::Equal))
                .ok_or("Error retrieving minimum score")?
                .0
                .clone();
            if (current_node == end_node) {
                node_history.push(current_node);
                break;
            }
        }
        Ok(node_history)
    }

    pub fn build_node_tree(
        width: usize,
        height: usize,
        bottom_left: (usize, usize),
        total_steps: usize,
        blocked_nodes: Vec<Node>,
    ) -> NodeTree {
        let gap_x = width / total_steps as usize;
        let gap_y = height / total_steps as usize;
        // TODO: panic if we hit usize limit?
        let mut node_tree: NodeTree = HashMap::with_capacity(total_steps * total_steps);
        for x in 0..total_steps {
            for y in 0..total_steps {
                let x_coord = bottom_left.0 + x * gap_x;
                let y_coord = bottom_left.1 + y * gap_y;
                let node: Node = (x_coord, y_coord);
                if blocked_nodes.contains(&node) {
                    continue;
                }
                let mut adjacent_nodes = vec![];

                if y_coord < height + bottom_left.1 {
                    let above = (x_coord, y_coord + gap_y);
                    if !blocked_nodes.contains(&above) {
                        adjacent_nodes.push(above);
                    }
                }

                if y_coord > bottom_left.1 {
                    let below = (x_coord, y_coord - gap_y);
                    if !blocked_nodes.contains(&below) {
                        adjacent_nodes.push(below);
                    }
                }

                if x_coord > bottom_left.0 {
                    let left = (x_coord - gap_x, y_coord);
                    if !blocked_nodes.contains(&left) {
                        adjacent_nodes.push(left);
                    }
                }

                if x_coord < bottom_left.0 + width {
                    let right = (x_coord + gap_x, y_coord);
                    if !blocked_nodes.contains(&right) {
                        adjacent_nodes.push(right);
                    }
                }

                node_tree.insert(node, adjacent_nodes);
            }
        }
        node_tree
    }

    #[cfg(test)]
    mod test {
        use crate::path_finding::{Node, a_star_path, build_node_tree};

        #[test]
        fn test_build_node_tree() {
            let node_tree = build_node_tree(10, 10, (0, 0), 10, vec![]);
            assert_eq!(node_tree.get(&(0, 0)).unwrap(), &vec![(0, 1), (1, 0)]);
            assert_eq!(
                node_tree.get(&(5, 5)).unwrap(),
                &vec![(5, 6), (5, 4), (4, 5), (6, 5)]
            );
        }

        #[test]
        fn test_find_path_without_blocked_nodes() {
            let node_tree = build_node_tree(10, 10, (0, 0), 10, vec![]);
            assert_eq!(node_tree.get(&(0, 0)).unwrap(), &vec![(0, 1), (1, 0)]);
            let path = a_star_path(node_tree, (0, 0), (1, 1));
            assert_eq!(path.unwrap().len(), 3);
        }

        #[test]
        fn test_find_path_with_one_blocked_node() {
            let node_tree = build_node_tree(10, 10, (0, 0), 10, vec![(0, 1)]);
            assert_eq!(node_tree.get(&(0, 0)).unwrap(), &vec![(0, 1), (1, 0)]);
            let path = a_star_path(node_tree, (0, 0), (1, 1));
            assert_eq!(path.as_ref().unwrap().len(), 3);
            assert_eq!(path.as_ref().unwrap(), &vec![(0, 0), (1, 0), (1, 1)]);
        }

        #[test]
        fn test_find_path_with_row_of_blocked_node() {
            let blocked_nodes = (0..8)
                .into_iter()
                .map(|x| return (x, 1))
                .collect::<Vec<Node>>();
            let node_tree = build_node_tree(10, 10, (0, 0), 10, blocked_nodes);
            assert_eq!(node_tree.get(&(0, 0)).unwrap(), &vec![(1, 0)]);
            let path = a_star_path(node_tree, (0, 0), (0, 2));
            assert_eq!(path.as_ref().unwrap().len(), 19);
        }

        #[test]
        fn test_find_path_with_row_of_blocked_node_and_bump() {
            let mut blocked_nodes = (0..8)
                .into_iter()
                .map(|x| return (x, 1))
                .collect::<Vec<Node>>();
            blocked_nodes.push((1, 2));
            let node_tree = build_node_tree(10, 10, (0, 0), 10, blocked_nodes);
            assert_eq!(node_tree.get(&(0, 0)).unwrap(), &vec![(1, 0)]);
            let path = a_star_path(node_tree, (0, 0), (0, 2));
            assert_eq!(path.as_ref().unwrap().len(), 21);
        }

        #[test]
        fn test_find_path_with_row_of_blocked_node_and_gap() {
            let mut blocked_nodes = (0..8)
                .into_iter()
                .map(|x| return (x, 1))
                .collect::<Vec<Node>>();
            blocked_nodes.remove(2);
            let node_tree = build_node_tree(10, 10, (0, 0), 10, blocked_nodes);
            assert_eq!(node_tree.get(&(0, 0)).unwrap(), &vec![(1, 0)]);
            let path = a_star_path(node_tree, (0, 0), (0, 2));
            assert_eq!(path.as_ref().unwrap().len(), 7);
        }
    }
}
