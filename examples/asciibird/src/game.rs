use hewn::ecs::{
    CameraFollow, EntityId, PositionComponent, RenderComponent, SizeComponent, VelocityComponent,
};
use hewn::ecs::{Components, ECS};
use hewn::game::GameHandler;
use hewn::runtime::Key;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;

pub const WIDTH: u16 = 1000;
pub const HEIGHT: u16 = 30;

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

    rng: Box<dyn rand::RngCore>,
    ecs: ECS,
    player_id: EntityId,
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
            ecs: ECS::new(),
            player_id: EntityId(0),
            wall_ids: HashSet::new(),
            width,
            height,
            rng,
        }
    }

    fn move_player(&mut self, key: Option<Key>) {
        if let Some(Key::Up) = key {
            if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
                if let Some(vel) = &mut player.components.velocity {
                    vel.y = 5;
                }
            }
        }
    }

    pub fn initialise_player(&mut self) {
        let components = Components {
            position: Some(PositionComponent { x: 1, y: 1 }),
            velocity: Some(VelocityComponent { x: 1, y: 5 }),
            size: Some(SizeComponent { x: 1, y: 1 }),
            render: Some(RenderComponent {
                ascii_character: '#',
            }),
            camera_follow: Some(CameraFollow {}),
        };
        let id = self.ecs.add_entity_from_components(components);
        self.player_id = id;
    }

    pub fn add_walls_from_positions(&mut self, walls: Vec<(u16, u16)>) {
        for (x, y) in walls.into_iter() {
            let components = Components {
                position: Some(PositionComponent { x, y }),
                velocity: Some(VelocityComponent { x: 0, y: 0 }),
                size: Some(SizeComponent { x: 1, y: 1 }),
                render: Some(RenderComponent {
                    ascii_character: '\\',
                }),
                camera_follow: None,
            };
            let id = self.ecs.add_entity_from_components(components);
            self.wall_ids.insert(id);
        }
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }

    pub fn initialise_walls(&mut self) {
        let mut walls: Vec<(u16, u16)> = vec![];
        let mut last_wall: u16 = 0;

        for index in 0..self.width {
            if last_wall > 8 {
                let y = self.rng.gen_range(0..(self.height.saturating_sub(5)));
                let wall_height = 5;
                for yy in y..(y + wall_height) {
                    walls.push((index as u16, yy as u16));
                }
                last_wall = 0;
            }

            if self.rng.gen_range(0..10) == 0 {
                let y = self.rng.gen_range(0..(self.height.saturating_sub(5)));
                let wall_height: u16 = 5;
                for yy in y..(y + wall_height) {
                    walls.push((index as u16, yy as u16));
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
        if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(pos) = &mut player.components.position {
                pos.x = 1;
                pos.y = 1;
            }
            if let Some(vel) = &mut player.components.velocity {
                vel.x = 1;
                vel.y = 5;
            }
        }
        self.state = GameState::InGame;
    }

    fn next(&mut self, key: Option<Key>) {
        if self.state != GameState::InGame {
            return;
        }

        self.move_player(key);

        self.ecs.step();

        if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(vel) = &mut player.components.velocity {
                vel.y -= 1;
            }
        }

        let collisions = self.ecs.collision_pass();
        for pair in collisions.into_iter() {
            let (a, b) = (pair[0], pair[1]);
            if (a == self.player_id && self.wall_ids.contains(&b))
                || (b == self.player_id && self.wall_ids.contains(&a))
            {
                self.end_game();
                break;
            }
        }

        if let Some(player) = self.ecs.get_entity_by_id(self.player_id) {
            if let Some(vel) = &player.components.velocity {
                if vel.y < -10 {
                    self.end_game();
                }
            }
        }

        if let Some(player) = self.ecs.get_entity_by_id(self.player_id) {
            if let Some(pos) = &player.components.position {
                self.score = self.score.max(pos.x as u16);
            }
        }
    }

    fn ecs(&self) -> &ECS {
        &self.ecs
    }

    fn debug_str(&self) -> Option<String> {
        if let Some(player) = self.ecs.get_entity_by_id(self.player_id) {
            let pos = player.components.position.as_ref()?;
            let vel = player.components.velocity.as_ref()?;
            Some(format!("v = {:4}, x = {:3}, y = {:3}", vel.y, pos.x, pos.y))
        } else {
            None
        }
    }
}
