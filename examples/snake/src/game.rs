use cgmath;
use hewn::ecs::{
    CameraFollow, EntityId, PositionComponent, RenderComponent, SizeComponent, VelocityComponent,
};
use hewn::ecs::{Components, ECS};
use hewn::runtime::{GameHandler, Key, RuntimeEvent};
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};
use std::collections::HashSet;
use std::time::Duration;

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

    // Add timer for discrete movement
    move_timer: f32,
    move_interval: f32,
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
            move_timer: 0.0,
            move_interval: 0.1,
        }
    }

    pub fn initialise_player(&mut self) {
        let components = Components {
            position: Some(PositionComponent { x: 1.0, y: 1.0 }),
            velocity: None,
            size: Some(SizeComponent { x: 1.0, y: 1.0 }),
            render: Some(RenderComponent {
                ascii_character: '0',
                rgb: cgmath::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            }),
            camera_follow: Some(CameraFollow {}),
        };
        let id = self.ecs.add_entity_from_components(components);
        self.player_id = id;
        self.player_direction = Direction::Up;
    }

    pub fn add_walls_from_positions(&mut self, walls: Vec<(f32, f32)>) {
        for (x, y) in walls.into_iter() {
            let components = Components {
                position: Some(PositionComponent { x, y }),
                velocity: None,
                size: Some(SizeComponent { x: 1.0, y: 1.0 }),
                render: Some(RenderComponent {
                    ascii_character: '#',
                    rgb: cgmath::Vector3 {
                        x: 0.0,
                        y: 0.1,
                        z: 0.0,
                    },
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

    fn move_head_discrete(&mut self) {
        if let Some(head) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(pos) = &mut head.components.position {
                match self.player_direction {
                    Direction::Left => {
                        pos.x -= 1.0;
                    }
                    Direction::Right => {
                        pos.x += 1.0;
                    }
                    Direction::Up => {
                        pos.y += 1.0;
                    }
                    Direction::Down => {
                        pos.y -= 1.0;
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
            velocity: None,
            size: Some(SizeComponent { x: 1.0, y: 1.0 }),
            render: Some(RenderComponent {
                ascii_character: '+',
                rgb: cgmath::Vector3 {
                    x: 0.1,
                    y: 0.0,
                    z: 0.0,
                },
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
                    size.x = 1.0;
                    size.y = 1.0;
                }
                if let Some(render) = &mut food.components.render {
                    render.ascii_character = '+';
                }
            }
        }
    }

    fn find_empty_tile(&mut self) -> Option<(f32, f32)> {
        let mut occupied: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
        let head_pos = self.head_position();
        occupied.insert((head_pos.0.round() as i32, head_pos.1.round() as i32));
        for (x, y) in self.body_positions() {
            occupied.insert((x.round() as i32, y.round() as i32));
        }
        for w in self.wall_ids.iter() {
            if let Some(ent) = self.ecs.get_entity_by_id(*w) {
                if let Some(pos) = &ent.components.position {
                    occupied.insert((pos.x.round() as i32, pos.y.round() as i32));
                }
            }
        }

        let rng = &mut self.rng;
        let mut target: Option<(f32, f32)> = None;
        let max_tries = (self.width as u32 * self.height as u32).max(100);
        for _ in 0..max_tries {
            let x_grid = rng.gen_range(1..(self.width - 1));
            let y_grid = rng.gen_range(1..(self.height - 1));
            let x = x_grid as f32;
            let y = y_grid as f32;
            if !occupied.contains(&(x as i32, y as i32)) {
                target = Some((x, y));
                break;
            }
        }
        target
    }

    fn grow_body_by_one(&mut self, tail_target: (f32, f32)) {
        let components = Components {
            position: Some(PositionComponent {
                x: tail_target.0,
                y: tail_target.1,
            }),
            velocity: None,
            size: Some(SizeComponent { x: 1.0, y: 1.0 }),
            render: Some(RenderComponent {
                ascii_character: 'o',
                rgb: cgmath::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.1,
                },
            }),
            camera_follow: None,
        };
        let id = self.ecs.add_entity_from_components(components);
        self.body_ids.push(id);
    }

    fn head_position(&self) -> (f32, f32) {
        let head = self.ecs.get_entity_by_id(self.player_id).unwrap();
        let pos = head.components.position.as_ref().unwrap();
        (pos.x, pos.y)
    }

    fn body_positions(&self) -> Vec<(f32, f32)> {
        self.body_ids
            .iter()
            .filter_map(|id| self.ecs.get_entity_by_id(*id))
            .filter_map(|e| e.components.position.as_ref())
            .map(|p| (p.x, p.y))
            .collect()
    }

    fn set_entity_position(&mut self, id: EntityId, xy: (f32, f32)) {
        if let Some(ent) = self.ecs.get_entity_by_id_mut(id) {
            if let Some(pos) = &mut ent.components.position {
                pos.x = xy.0;
                pos.y = xy.1;
            }
        }
    }

    fn update_body_segments(&mut self, prev_positions: Vec<(f32, f32)>) {
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
        self.move_timer = 0.0;
        if let Some(head) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(pos) = &mut head.components.position {
                pos.x = 1.0;
                pos.y = 1.0;
            }
        }
        self.player_direction = Direction::Up;
        self.state = GameState::InGame;
    }

    fn handle_event(&mut self, event: RuntimeEvent) -> bool {
        match event {
            RuntimeEvent::Key(key_event) => {
                if key_event.pressed {
                    self.player_direction =
                        Game::compute_next_direction(self.player_direction, Some(key_event.key));
                }
                true
            }
            _ => false,
        }
    }

    fn next(&mut self, dt: Duration) {
        if self.state != GameState::InGame {
            return;
        }

        // Update move timer
        self.move_timer += dt.as_secs_f32();

        // Only move when timer reaches the interval
        if self.move_timer >= self.move_interval {
            self.move_timer = 0.0; // Reset timer

            // Store previous positions for body segments
            let mut prev_positions: Vec<(f32, f32)> = Vec::with_capacity(self.body_ids.len() + 1);
            prev_positions.push(self.head_position());
            prev_positions.extend(self.body_positions());

            // Move the head one discrete step
            self.move_head_discrete();

            // Check for collisions after discrete movement
            let head_pos = self.head_position();
            let mut ate_food = false;

            // Check wall collisions
            for wall_id in &self.wall_ids {
                if let Some(wall) = self.ecs.get_entity_by_id(*wall_id) {
                    if let Some(wall_pos) = &wall.components.position {
                        if (head_pos.0 - wall_pos.x).abs() < 1.0
                            && (head_pos.1 - wall_pos.y).abs() < 1.0
                        {
                            self.end_game();
                            return;
                        }
                    }
                }
            }

            // Check body collisions
            for body_id in &self.body_ids {
                if let Some(body) = self.ecs.get_entity_by_id(*body_id) {
                    if let Some(body_pos) = &body.components.position {
                        if (head_pos.0 - body_pos.x).abs() < 1.0
                            && (head_pos.1 - body_pos.y).abs() < 1.0
                        {
                            self.end_game();
                            return;
                        }
                    }
                }
            }

            // Check food collision
            if let Some(food_id) = self.food_id {
                if let Some(food) = self.ecs.get_entity_by_id(food_id) {
                    if let Some(food_pos) = &food.components.position {
                        if (head_pos.0 - food_pos.x).abs() < 1.0
                            && (head_pos.1 - food_pos.y).abs() < 1.0
                        {
                            ate_food = true;
                        }
                    }
                }
            }

            // Handle food consumption first (before updating body segments)
            if ate_food {
                let tail_target = self
                    .body_ids
                    .last()
                    .and_then(|id| self.ecs.get_entity_by_id(*id))
                    .and_then(|e| e.components.position.as_ref().map(|p| (p.x, p.y)))
                    .unwrap_or_else(|| prev_positions.first().copied().unwrap_or((10.0, 10.0)));
                self.grow_body_by_one(tail_target);
                self.spawn_food();
            }

            // Update body segments
            self.update_body_segments(prev_positions);

            let length = 1 + self.body_ids.len() as u16;
            self.score = self.score.max(length);
        }
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

fn generate_walls_positions(width: u16, height: u16) -> Vec<(f32, f32)> {
    let mut walls: Vec<(f32, f32)> = vec![];
    for x_index in 0..width {
        walls.push((x_index as f32, 1.0));
        walls.push((x_index as f32, height as f32));
    }
    for y_index in 1..(height) {
        walls.push((0.0, y_index as f32));
        walls.push(((width - 1) as f32, y_index as f32));
    }
    walls
}
