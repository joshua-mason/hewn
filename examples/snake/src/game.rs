use hewn::ecs::{
    CameraFollow, EntityId, PositionComponent, RenderComponent, SizeComponent, VelocityComponent,
};
use hewn::ecs::{Components, ECS};
use hewn::runtime::{GameHandler, Key};
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};
use std::collections::HashSet;

pub fn create_game(width: u16, height: u16, seed: Option<u64>) -> Game {
    let mut game = Game::new(width, height, seed);
    game.initialise_walls();
    game.initialise_player();
    game.initialise_food().expect(
        "Map should contain empty tile on initialisation. Check width and height arguments.",
    );
    game
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InGame,
    Menu,
    Lost(u16),
}

pub struct Game {
    pub width: u16,
    pub height: u16,
    pub score: u16,

    state: GameState,
    ecs: ECS,
    player_id: EntityId,
    player_direction: Direction,
    body_ids: Vec<EntityId>,
    wall_ids: HashSet<EntityId>,
    food_id: Option<EntityId>,
    rng: Box<dyn RngCore>,
}

impl Game {
    fn compute_next_direction(current: Direction, key: Option<Key>) -> Direction {
        let Some(key) = key else { return current };
        let proposed = match key {
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            _ => None,
        };
        if let Some(dir) = proposed {
            let is_uturn = matches!(
                (current, dir),
                (Direction::Left, Direction::Right)
                    | (Direction::Right, Direction::Left)
                    | (Direction::Up, Direction::Down)
                    | (Direction::Down, Direction::Up)
            );
            if !is_uturn {
                return dir;
            }
        }
        current
    }

    pub fn new(width: u16, height: u16, seed: Option<u64>) -> Game {
        let rng: Box<dyn RngCore> = if let Some(s) = seed {
            Box::new(StdRng::seed_from_u64(s))
        } else {
            Box::new(rand::thread_rng())
        };
        Game {
            width,
            height,
            state: GameState::Menu,
            score: 0,
            ecs: ECS::new(),
            player_id: EntityId(0),
            player_direction: Direction::Up,
            body_ids: vec![],
            wall_ids: HashSet::new(),
            food_id: None,
            rng,
        }
    }

    pub fn initialise_player(&mut self) {
        let components = Components {
            position: Some(PositionComponent { x: 1, y: 1 }),
            velocity: Some(VelocityComponent { x: 0, y: 1 }),
            size: Some(SizeComponent { x: 1, y: 1 }),
            render: Some(RenderComponent {
                ascii_character: '0',
            }),
            camera_follow: Some(CameraFollow {}),
        };
        let id = self.ecs.add_entity_from_components(components);
        self.player_id = id;
        self.player_direction = Direction::Up;
    }

    pub fn add_walls_from_positions(&mut self, walls: Vec<(u16, u16)>) {
        for (x, y) in walls.into_iter() {
            let components = Components {
                position: Some(PositionComponent { x, y }),
                velocity: Some(VelocityComponent { x: 0, y: 0 }),
                size: Some(SizeComponent { x: 1, y: 1 }),
                render: Some(RenderComponent {
                    ascii_character: '#',
                }),
                camera_follow: None,
            };
            let id = self.ecs.add_entity_from_components(components);
            self.wall_ids.insert(id);
        }
    }

    pub fn initialise_walls(&mut self) {
        let walls_positions = generate_walls_positions(self.width, self.height);
        self.add_walls_from_positions(walls_positions);
    }

    fn set_head_velocity_from_direction(&mut self) {
        if let Some(head) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(vel) = &mut head.components.velocity {
                match self.player_direction {
                    Direction::Left => {
                        vel.x = -1;
                        vel.y = 0;
                    }
                    Direction::Right => {
                        vel.x = 1;
                        vel.y = 0;
                    }
                    Direction::Up => {
                        vel.x = 0;
                        vel.y = 1;
                    }
                    Direction::Down => {
                        vel.x = 0;
                        vel.y = -1;
                    }
                }
            }
        }
    }

    pub fn initialise_food(&mut self) -> Result<(), &str> {
        let empty_tile = self.find_empty_tile().ok_or("No empty tile")?;
        let components = Components {
            position: Some(PositionComponent {
                x: empty_tile.0,
                y: empty_tile.1,
            }),
            velocity: Some(VelocityComponent { x: 0, y: 0 }),
            size: Some(SizeComponent { x: 1, y: 1 }),
            render: Some(RenderComponent {
                ascii_character: '+',
            }),
            camera_follow: None,
        };
        let id = self.ecs.add_entity_from_components(components);
        self.food_id = Some(id);
        Ok(())
    }

    pub fn spawn_food(&mut self) {
        let target = self.find_empty_tile();

        if let (Some((x, y)), Some(fid)) = (target, self.food_id) {
            if let Some(food) = self.ecs.get_entity_by_id_mut(fid) {
                if let Some(pos) = &mut food.components.position {
                    pos.x = x;
                    pos.y = y;
                }
                if let Some(size) = &mut food.components.size {
                    size.x = 1;
                    size.y = 1;
                }
                if let Some(render) = &mut food.components.render {
                    render.ascii_character = '+';
                }
            }
        }
    }

    fn find_empty_tile(&mut self) -> Option<(u16, u16)> {
        let mut occupied: std::collections::HashSet<(u16, u16)> = std::collections::HashSet::new();
        occupied.insert(self.head_position());
        for (x, y) in self.body_positions() {
            occupied.insert((x, y));
        }
        for w in self.wall_ids.iter() {
            if let Some(ent) = self.ecs.get_entity_by_id(*w) {
                if let Some(pos) = &ent.components.position {
                    occupied.insert((pos.x, pos.y));
                }
            }
        }

        let rng = &mut self.rng;
        let mut target: Option<(u16, u16)> = None;
        let max_tries = (self.width as u32 * self.height as u32).max(100);
        for _ in 0..max_tries {
            let x = rng.gen_range(1..(self.width - 1));
            let y = rng.gen_range(1..(self.height - 1));
            if !occupied.contains(&(x, y)) {
                target = Some((x, y));
                break;
            }
        }
        target
    }

    fn grow_body_by_one(&mut self, tail_target: (u16, u16)) {
        let components = Components {
            position: Some(PositionComponent {
                x: tail_target.0,
                y: tail_target.1,
            }),
            velocity: Some(VelocityComponent { x: 0, y: 0 }),
            size: Some(SizeComponent { x: 1, y: 1 }),
            render: Some(RenderComponent {
                ascii_character: 'o',
            }),
            camera_follow: None,
        };
        let id = self.ecs.add_entity_from_components(components);
        self.body_ids.push(id);
    }

    fn head_position(&self) -> (u16, u16) {
        let head = self.ecs.get_entity_by_id(self.player_id).unwrap();
        let pos = head.components.position.as_ref().unwrap();
        (pos.x, pos.y)
    }

    fn body_positions(&self) -> Vec<(u16, u16)> {
        self.body_ids
            .iter()
            .filter_map(|id| self.ecs.get_entity_by_id(*id))
            .filter_map(|e| e.components.position.as_ref())
            .map(|p| (p.x, p.y))
            .collect()
    }

    fn set_entity_position(&mut self, id: EntityId, xy: (u16, u16)) {
        if let Some(ent) = self.ecs.get_entity_by_id_mut(id) {
            if let Some(pos) = &mut ent.components.position {
                pos.x = xy.0;
                pos.y = xy.1;
            }
        }
    }

    fn update_body_segments(&mut self, prev_positions: Vec<(u16, u16)>) {
        let ids = self.body_ids.clone();
        for (i, id) in ids.iter().enumerate() {
            if i < prev_positions.len() {
                self.set_entity_position(*id, prev_positions[i]);
            }
        }
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }
}

impl GameHandler for Game {
    fn start_game(&mut self) {
        self.score = 0;
        if let Some(head) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(pos) = &mut head.components.position {
                pos.x = 1;
                pos.y = 1;
            }
        }
        self.player_direction = Direction::Up;
        self.state = GameState::InGame;
    }

    fn handle_key(&mut self, key: Key, pressed: bool) -> bool {
        if pressed {
            self.player_direction = Game::compute_next_direction(self.player_direction, Some(key));
        }
        true
    }

    fn next(&mut self) {
        if self.state != GameState::InGame {
            return;
        }

        let mut prev_positions: Vec<(u16, u16)> = Vec::with_capacity(self.body_ids.len() + 1);
        prev_positions.push(self.head_position());
        prev_positions.extend(self.body_positions());

        // let next_dir = Game::compute_next_direction(self.player_direction, key);
        // self.player_direction = next_dir;
        self.set_head_velocity_from_direction();

        self.ecs.step();

        let collisions = self.ecs.collision_pass();
        let mut ate_food = false;

        for [a, b] in collisions.into_iter() {
            let (_player, other) = if a == self.player_id {
                (a, b)
            } else if b == self.player_id {
                (b, a)
            } else {
                continue;
            };

            if self.wall_ids.contains(&other) || self.body_ids.contains(&other) {
                self.end_game();
                return;
            }

            if let Some(fid) = self.food_id {
                if other == fid {
                    ate_food = true;
                }
            }
        }

        self.update_body_segments(prev_positions);

        if ate_food {
            let tail_target = self
                .body_ids
                .last()
                .and_then(|id| self.ecs.get_entity_by_id(*id))
                .and_then(|e| e.components.position.as_ref().map(|p| (p.x, p.y)))
                .unwrap_or_else(|| self.head_position());
            self.grow_body_by_one(tail_target);

            self.spawn_food();
        }

        let length = 1 + self.body_ids.len() as u16;
        self.score = self.score.max(length);
    }

    fn ecs(&self) -> &ECS {
        &self.ecs
    }

    fn debug_str(&self) -> Option<String> {
        if let Some(head) = self.ecs.get_entity_by_id(self.player_id) {
            let pos = head.components.position.as_ref()?;
            Some(format!(
                "len = {:3}, x = {:3}, y = {:3}, dir = {:?}",
                1 + self.body_ids.len(),
                pos.x,
                pos.y,
                self.player_direction
            ))
        } else {
            None
        }
    }
}

fn generate_walls_positions(width: u16, height: u16) -> Vec<(u16, u16)> {
    let mut walls: Vec<(u16, u16)> = vec![];
    for x_index in 0..width {
        walls.push((x_index, 1));
        walls.push((x_index, height));
    }
    for y_index in 0..height {
        walls.push((0, y_index));
        walls.push((width - 1, y_index));
    }
    walls
}
