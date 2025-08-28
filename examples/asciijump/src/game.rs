use hewn::ecs::{
    CameraFollow, EntityId, PositionComponent, RenderComponent, SizeComponent, VelocityComponent,
};
use hewn::ecs::{Components, ECS};
use hewn::game::GameHandler;
use hewn::runtime::Key;
use rand::RngCore;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;

pub const WIDTH: u16 = 10;
pub const HEIGHT: u16 = 500;
pub const SCREEN_WIDTH: u16 = 10;
pub const SCREEN_HEIGHT: u16 = 20;

pub fn create_game(seed: Option<u64>) -> Game {
    let mut game = Game::new(WIDTH, HEIGHT, seed);
    game.initialise_player();
    game.initialise_platforms();
    game
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
    pub state: GameState,
    pub score: u16,

    rng: Box<dyn RngCore>,
    ecs: ECS,
    player_id: EntityId,
    platform_ids: HashSet<EntityId>,
}

impl Game {
    pub fn new(width: u16, height: u16, seed: Option<u64>) -> Game {
        let rng: Box<dyn rand::RngCore> = if let Some(s) = seed {
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
            platform_ids: HashSet::new(),
            rng,
        }
    }

    fn move_player(&mut self, key: Option<Key>) {
        match key {
            Some(Key::Left) => {
                if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
                    if let Some(pos) = &mut player.components.position {
                        if pos.x > 0 {
                            pos.x -= 1;
                        }
                    }
                }
            }
            Some(Key::Right) => {
                if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
                    if let Some(pos) = &mut player.components.position {
                        if pos.x < self.width - 1 {
                            pos.x += 1;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }

    pub fn initialise_player(&mut self) {
        let components = Components {
            position: Some(PositionComponent { x: 1, y: 1 }),
            velocity: Some(VelocityComponent { x: 0, y: 5 }),
            size: Some(SizeComponent { x: 1, y: 1 }),
            render: Some(RenderComponent {
                ascii_character: '#',
            }),
            camera_follow: Some(CameraFollow {}),
        };
        let id = self.ecs.add_entity_from_components(components);
        self.player_id = id;
    }

    pub fn add_platforms_from_positions(&mut self, platforms: Vec<(u16, u16)>) {
        for (x, y) in platforms.into_iter() {
            let components = Components {
                position: Some(PositionComponent { x, y }),
                velocity: Some(VelocityComponent { x: 0, y: 0 }),
                size: Some(SizeComponent { x: 3, y: 1 }),
                render: Some(RenderComponent {
                    ascii_character: '=',
                }),
                camera_follow: None,
            };
            let id = self.ecs.add_entity_from_components(components);
            self.platform_ids.insert(id);
        }
    }

    pub fn initialise_platforms(&mut self) {
        let mut platforms: Vec<(u16, u16)> = vec![];
        let mut last_platform: usize = 0;
        for index in 0..self.height {
            if last_platform > 8 {
                let x = self.rng.gen_range(0..(self.width - 3));
                platforms.push((x as u16, index as u16));
                last_platform = 0;
            }
            if self.rng.gen_range(0..10) == 0 {
                let x = self.rng.gen_range(0..(self.width - 3));
                platforms.push((x as u16, index as u16));
                last_platform = 0;
            }
            last_platform += 1;
        }
        self.add_platforms_from_positions(platforms);
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
        for [a, b] in collisions.into_iter() {
            let (player, other) = if a == self.player_id {
                (a, b)
            } else if b == self.player_id {
                (b, a)
            } else {
                continue;
            };
            if self.platform_ids.contains(&other) {
                let platform_y = self
                    .ecs
                    .get_entity_by_id(other)
                    .and_then(|e| e.components.position.as_ref().map(|p| p.y));

                if let Some(p) = self.ecs.get_entity_by_id_mut(player) {
                    if let Some(vel) = &mut p.components.velocity {
                        if vel.y < 1 {
                            vel.y = 5;
                        }
                    }
                    if let (Some(pos), Some(py)) = (&mut p.components.position, platform_y) {
                        pos.y = py;
                    }
                }
            }
        }

        let mut should_end = false;
        let mut maybe_new_score: Option<u16> = None;
        if let Some(player) = self.ecs.get_entity_by_id(self.player_id) {
            if let Some(vel) = &player.components.velocity {
                if vel.y < -6 {
                    should_end = true;
                }
            }
            if let Some(pos) = &player.components.position {
                if pos.y as u16 > self.height {
                    should_end = true;
                }
                maybe_new_score = Some(pos.y);
            }
        }
        if should_end {
            self.end_game();
        }
        if let Some(s) = maybe_new_score {
            self.score = self.score.max(s);
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

#[cfg(test)]
mod tests {
    use super::*;
    use hewn::ecs::ComponentType;
    use hewn::game::GameHandler;

    fn get_player_entity<'a>(game: &'a Game) -> &'a hewn::ecs::Entity {
        let ecs = game.ecs();
        let mut tracked = ecs.get_entities_by(ComponentType::CameraFollow);
        assert!(tracked.len() > 0, "player entity not found");
        tracked.remove(0)
    }

    #[test]
    fn ignore_collision_when_moving_up() {
        let mut game = Game::new(10, 10, Some(42));
        game.initialise_player();
        game.add_platforms_from_positions(vec![(1, 3)]);
        game.start_game();

        game.next(None);

        let player = get_player_entity(&game);
        let pos = player.components.position.as_ref().unwrap();
        let vel = player.components.velocity.as_ref().unwrap();

        assert_eq!(
            pos.y, 6,
            "position should not snap to platform while moving up"
        );
        assert_eq!(vel.y, 4, "velocity should decrease by gravity only");
    }

    #[test]
    fn bounce_when_falling_onto_platform() {
        let mut game = Game::new(10, 20, Some(42));
        game.initialise_player();
        game.add_platforms_from_positions(vec![(1, 5)]);
        game.start_game();

        let mut bounced = false;
        for _ in 0..30 {
            game.next(None);
            let player = get_player_entity(&game);
            let pos = player.components.position.as_ref().unwrap();
            let vel = player.components.velocity.as_ref().unwrap();
            if pos.y == 5 && vel.y == 5 {
                bounced = true;
                break;
            }
        }
        assert!(bounced, "expected to bounce on the platform when falling");
    }
}
