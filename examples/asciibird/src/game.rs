use hewn::runtime::GameHandler;
use hewn::runtime::Key;
use hewn::scene::{
    CameraFollow, EntityId, PositionComponent, RenderComponent, SizeComponent, VelocityComponent,
};
use hewn::scene::{Components, Scene};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::time::Duration;

pub const WIDTH: u16 = 1000;
pub const HEIGHT: u16 = 30;
const JUMP_VELOCITY: f32 = 30.0;
const HORIZONTAL_VELOCITY: f32 = 7.0;
const GRAVITY_MODIFIER: f32 = 30.0;
const STARTING_WALL_X_POS: u16 = 15;
const WALL_HEIGHT: u16 = 5;
const END_Y_POS: f32 = -10.0;

pub fn create_game(seed: Option<u64>) -> Game {
    let mut game = Game::new(WIDTH as u16, HEIGHT as u16, seed);
    game.initialise_walls();
    game.initialise_player();
    game
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InGame,
    Lost(u16),
}

pub struct Game {
    pub state: GameState,
    pub score: u16,
    pub player_id: EntityId,

    rng: Box<dyn rand::RngCore>,
    scene: Scene,
    wall_ids: HashSet<EntityId>,
    width: u16,
    height: u16,
}

impl Game {
    pub fn new(width: u16, height: u16, seed: Option<u64>) -> Game {
        let rng: Box<dyn rand::RngCore> = if let Some(s) = seed {
            Box::new(StdRng::seed_from_u64(s))
        } else {
            Box::new(rand::thread_rng())
        };

        Game {
            state: GameState::InGame,
            score: 0,
            scene: Scene::new(),
            player_id: EntityId(0),
            wall_ids: HashSet::new(),
            width,
            height,
            rng,
        }
    }

    pub fn initialise_player(&mut self) {
        let components = Components {
            position: Some(PositionComponent { x: 1.0, y: 1.0 }),
            velocity: Some(VelocityComponent {
                x: HORIZONTAL_VELOCITY,
                y: JUMP_VELOCITY,
            }),
            size: Some(SizeComponent { x: 1.0, y: 1.0 }),
            render: Some(RenderComponent { sprite_tile: None,
                ascii_character: '#',
                rgb: (0.0, 0.0, 0.0).into(),
            }),
            camera_follow: Some(CameraFollow {}),
        };
        let id = self.scene.add_entity_from_components(components);
        self.player_id = id;
    }

    pub fn add_walls_from_positions(&mut self, walls: Vec<(f32, f32)>) {
        for (x, y) in walls.into_iter() {
            let components = Components {
                position: Some(PositionComponent { x, y }),
                velocity: Some(VelocityComponent { x: 0.0, y: 0.0 }),
                size: Some(SizeComponent { x: 1.0, y: 1.0 }),
                render: Some(RenderComponent { sprite_tile: None,
                    ascii_character: '\\',
                    rgb: (0.0, 0.0, 0.5).into(),
                }),
                camera_follow: None,
            };
            let id = self.scene.add_entity_from_components(components);
            self.wall_ids.insert(id);
        }
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }

    pub fn initialise_walls(&mut self) {
        let mut walls: Vec<(f32, f32)> = vec![];
        let mut last_wall: u16 = 0;

        for index in STARTING_WALL_X_POS..self.width {
            if last_wall > 8 {
                let y = self
                    .rng
                    .gen_range(0..(self.height.saturating_sub(WALL_HEIGHT)));
                for yy in y..(y + WALL_HEIGHT) {
                    walls.push((index as f32, yy as f32));
                }
                last_wall = 0;
            }

            if self.rng.gen_range(0..10) == 0 {
                let y = self
                    .rng
                    .gen_range(0..(self.height.saturating_sub(WALL_HEIGHT)));
                for yy in y..(y + WALL_HEIGHT as u16) {
                    walls.push((index as f32, yy as f32));
                }
                last_wall = 0;
            }
            last_wall += 1;
        }
        self.add_walls_from_positions(walls);
    }
}

impl GameHandler for Game {
    fn start_game(&mut self) {
        self.score = 0;
        if let Some(player) = self.scene.get_entity_by_id_mut(self.player_id) {
            if let Some(pos) = &mut player.components.position {
                pos.x = 1.0;
                pos.y = 1.0;
            }
            if let Some(vel) = &mut player.components.velocity {
                vel.x = HORIZONTAL_VELOCITY;
                vel.y = JUMP_VELOCITY;
            }
        }
        self.state = GameState::InGame;
    }

    fn next(&mut self, dt: Duration) {
        if self.state != GameState::InGame {
            return;
        }

        if let Some(player) = self.scene.get_entity_by_id_mut(self.player_id) {
            if let Some(vel) = &mut player.components.velocity {
                vel.y -= GRAVITY_MODIFIER * dt.as_secs_f32();
            }
        }

        let collisions = self.scene.collision_pass(dt);
        for pair in collisions.into_iter() {
            let (a, b) = (pair[0], pair[1]);
            if (a == self.player_id && self.wall_ids.contains(&b))
                || (b == self.player_id && self.wall_ids.contains(&a))
            {
                self.end_game();
                break;
            }
        }

        if let Some(player) = self.scene.get_entity_by_id(self.player_id) {
            if let Some(pos) = &player.components.position {
                if pos.y < END_Y_POS {
                    self.end_game();
                }
            }
        }

        if let Some(player) = self.scene.get_entity_by_id(self.player_id) {
            if let Some(pos) = &player.components.position {
                self.score = self.score.max(pos.x as u16);
            }
        }

        self.scene.step(dt);
    }

    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn debug_str(&self) -> Option<String> {
        if let Some(player) = self.scene.get_entity_by_id(self.player_id) {
            let pos = player.components.position.as_ref()?;
            let vel = player.components.velocity.as_ref()?;
            Some(format!("v = {:4}, x = {:3}, y = {:3}", vel.y, pos.x, pos.y))
        } else {
            None
        }
    }

    fn handle_key(&mut self, key: Key, pressed: bool) -> bool {
        if !pressed {
            return true; // Ignore key releases
        }

        match key {
            Key::Up => {
                if self.state == GameState::InGame {
                    if let Some(player) = self.scene.get_entity_by_id_mut(self.player_id) {
                        if let Some(velocity) = &mut player.components.velocity {
                            velocity.y = JUMP_VELOCITY;
                        }
                    }
                }
                true
            }
            Key::Space => {
                if self.state != GameState::InGame {
                    self.start_game();
                }
                true
            }
            _ => false,
        }
    }
}
