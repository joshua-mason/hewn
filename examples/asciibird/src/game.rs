use hewn::ecs::{Components, ECS};
use hewn::ecs::{
    EntityId, PositionComponent, RenderComponent, SizeComponent, TrackComponent, VelocityComponent,
};
use hewn::game::GameLogic;
use hewn::runtime::Key;
use rand::Rng;
use std::collections::HashSet;

pub const WIDTH: u16 = 1000;
pub const HEIGHT: u16 = 30;

pub fn create_game() -> Game {
    let mut game = Game::new(WIDTH as u16, HEIGHT as u16);
    let walls = generate_wall_positions(WIDTH, HEIGHT);
    game.add_player_from_position((1, 1));
    game.add_walls_from_positions(walls);
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
    ecs: ECS,
    player_id: EntityId,
    wall_ids: HashSet<EntityId>,
    width: u16,
    height: u16,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            state: GameState::InGame,
            score: 0,
            ecs: ECS::new(),
            player_id: EntityId(0),
            wall_ids: HashSet::new(),
            width,
            height,
        }
    }

    fn move_player(&mut self, key: Option<Key>) {
        if let Some(Key::Up) = key {
            if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
                if let Some(vel) = &mut player.components.velocity_component {
                    vel.y = 5;
                }
            }
        }
    }

    pub fn add_player_from_position(&mut self, start: (u16, u16)) {
        let components = Components {
            position_component: Some(PositionComponent {
                x: start.0,
                y: start.1,
            }),
            velocity_component: Some(VelocityComponent { x: 1, y: 5 }),
            size_component: Some(SizeComponent { x: 1, y: 1 }),
            render_component: Some(RenderComponent {
                ascii_character: '#',
            }),
            track_component: Some(TrackComponent {}),
        };
        let id = self.ecs.add_entity_from_components(components);
        self.player_id = id;
    }

    pub fn add_walls_from_positions(&mut self, walls: Vec<(u16, u16)>) {
        for (x, y) in walls.into_iter() {
            let components = Components {
                position_component: Some(PositionComponent { x, y }),
                velocity_component: Some(VelocityComponent { x: 0, y: 0 }),
                size_component: Some(SizeComponent { x: 1, y: 1 }),
                render_component: Some(RenderComponent {
                    ascii_character: '\\',
                }),
                track_component: None,
            };
            let id = self.ecs.add_entity_from_components(components);
            self.wall_ids.insert(id);
        }
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }
}

impl GameLogic for Game {
    fn start_game(&mut self) {
        self.score = 0;
        if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(pos) = &mut player.components.position_component {
                pos.x = 1;
                pos.y = 1;
            }
            if let Some(vel) = &mut player.components.velocity_component {
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
            if let Some(vel) = &mut player.components.velocity_component {
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
            if let Some(vel) = &player.components.velocity_component {
                if vel.y < -10 {
                    self.end_game();
                }
            }
        }

        if let Some(player) = self.ecs.get_entity_by_id(self.player_id) {
            if let Some(pos) = &player.components.position_component {
                self.score = self.score.max(pos.x as u16);
            }
        }
    }

    fn ecs(&self) -> &ECS {
        &self.ecs
    }

    fn debug_str(&self) -> Option<String> {
        if let Some(player) = self.ecs.get_entity_by_id(self.player_id) {
            let pos = player.components.position_component.as_ref()?;
            let vel = player.components.velocity_component.as_ref()?;
            Some(format!("v = {:4}, x = {:3}, y = {:3}", vel.y, pos.x, pos.y))
        } else {
            None
        }
    }
}

fn generate_wall_positions(width: u16, height: u16) -> Vec<(u16, u16)> {
    let mut walls: Vec<(u16, u16)> = vec![];
    let mut last_wall: u16 = 0;
    let mut rng = rand::thread_rng();

    for index in 0..width {
        if last_wall > 8 {
            let y = rng.gen_range(0..(height.saturating_sub(5)));
            let wall_height = 5;
            for yy in y..(y + wall_height) {
                walls.push((index as u16, yy as u16));
            }
            last_wall = 0;
        }

        if rng.gen_range(0..10) == 0 {
            let y = rng.gen_range(0..(height.saturating_sub(5)));
            let wall_height: u16 = 5;
            for yy in y..(y + wall_height) {
                walls.push((index as u16, yy as u16));
            }
            last_wall = 0;
        }
        last_wall += 1;
    }

    walls
}
