use hewn::ecs::{
    CameraFollow, EntityId, PositionComponent, RenderComponent, SizeComponent, VelocityComponent,
};
use hewn::ecs::{Components, ECS};
use hewn::runtime::GameHandler;
use hewn::runtime::Key;
use rand::RngCore;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::time::Duration;

pub const WIDTH: f32 = 10.0;
pub const HEIGHT: f32 = 500.0;
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
    pub width: f32,
    pub height: f32,
    pub state: GameState,
    pub score: u16,
    pub player_id: EntityId,

    rng: Box<dyn RngCore>,
    ecs: ECS,
    platform_ids: HashSet<EntityId>,
}

impl Game {
    pub fn new(width: f32, height: f32, seed: Option<u64>) -> Game {
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

    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }

    pub fn initialise_player(&mut self) {
        let components = Components {
            position: Some(PositionComponent { x: 1.0, y: 1.0 }),
            velocity: Some(VelocityComponent { x: 0.0, y: 5.0 }),
            size: Some(SizeComponent { x: 1.0, y: 1.0 }),
            render: Some(RenderComponent {
                ascii_character: '#',
                rgb: (0.0, 0.0, 0.0).into(),
            }),
            camera_follow: Some(CameraFollow {}),
        };
        let id = self.ecs.add_entity_from_components(components);
        self.player_id = id;
    }

    pub fn add_platforms_from_positions(&mut self, platforms: Vec<(f32, f32)>) {
        for (x, y) in platforms.into_iter() {
            let components = Components {
                position: Some(PositionComponent { x, y }),
                velocity: Some(VelocityComponent { x: 0.0, y: 0.0 }),
                size: Some(SizeComponent { x: 3.0, y: 1.0 }),
                render: Some(RenderComponent {
                    ascii_character: '=',
                    rgb: (0.0, 0.0, 0.5).into(),
                }),
                camera_follow: None,
            };
            let id = self.ecs.add_entity_from_components(components);
            self.platform_ids.insert(id);
        }
    }

    pub fn initialise_platforms(&mut self) {
        let mut platforms: Vec<(f32, f32)> = vec![];
        let mut last_platform: usize = 0;
        for index in 0..self.height as u16 {
            if last_platform > 8 {
                let x = self.rng.gen_range(0..(self.width as u16 - 3));
                platforms.push((x as f32, index as f32));
                last_platform = 0;
            }
            if self.rng.gen_range(0..10) == 0 {
                let x = self.rng.gen_range(0..(self.width as u16 - 3));
                platforms.push((x as f32, index as f32));
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
                pos.x = 1.0;
                pos.y = 1.0;
            }
            if let Some(vel) = &mut player.components.velocity {
                vel.y = 50.0;
            }
        }
        self.state = GameState::InGame;
    }

    fn next(&mut self, dt: Duration) {
        if self.state != GameState::InGame {
            return;
        }

        if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
            if let Some(vel) = &mut player.components.velocity {
                vel.y -= 100.0 * dt.as_secs_f32();
            }
        }

        let collisions = self.ecs.collision_pass(dt);
        for [a, b] in collisions.into_iter() {
            let (player, other) = if a == self.player_id {
                (a, b)
            } else if b == self.player_id {
                (b, a)
            } else {
                continue;
            };
            if self.platform_ids.contains(&other) {
                if let Some(p) = self.ecs.get_entity_by_id_mut(player) {
                    if let Some(vel) = &mut p.components.velocity {
                        if vel.y < 0.0 {
                            vel.y = 50.0;
                        }
                    }
                }
            }
        }

        let mut should_end = false;
        let mut maybe_new_score: Option<u16> = None;
        if let Some(player) = self.ecs.get_entity_by_id(self.player_id) {
            if let Some(pos) = &player.components.position {
                if pos.y < 0.0 {
                    should_end = true;
                }
            }
            if let Some(pos) = &player.components.position {
                if pos.y > self.height {
                    should_end = true;
                }
                maybe_new_score = Some(pos.y as u16);
            }
        }
        if should_end {
            self.end_game();
        }
        if let Some(s) = maybe_new_score {
            self.score = self.score.max(s);
        }
        self.ecs.step(dt);
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

    fn handle_event(&mut self, event: hewn::runtime::RuntimeEvent) -> bool {
        match event {
            hewn::runtime::RuntimeEvent::Key(hewn::runtime::KeyEvent { key, pressed }) => {
                match key {
                    Key::Left | Key::Right => {
                        if self.state == GameState::InGame {
                            if let Some(player) = self.ecs.get_entity_by_id_mut(self.player_id) {
                                if let Some(velocity) = &mut player.components.velocity {
                                    if pressed {
                                        velocity.x = if key == Key::Left { -10.0 } else { 10.0 };
                                    } else {
                                        // Only stop horizontal movement if the released key matches the current direction
                                        if (key == Key::Left && velocity.x < 0.0)
                                            || (key == Key::Right && velocity.x > 0.0)
                                        {
                                            velocity.x = 0.0;
                                        }
                                    }
                                }
                            }
                        }
                        true
                    }
                    Key::Space => {
                        if pressed {
                            self.start_game();
                        }
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hewn::ecs::ComponentType;
    use hewn::runtime::GameHandler;

    fn get_player_entity<'a>(game: &'a Game) -> &'a hewn::ecs::Entity {
        let ecs = game.ecs();
        let mut tracked = ecs.get_entities_with_component(ComponentType::CameraFollow);
        assert!(tracked.len() > 0, "player entity not found");
        tracked.remove(0)
    }

    #[test]
    fn ignore_collision_when_moving_up() {
        let mut game = Game::new(10.0, 10.0, Some(42));
        game.initialise_player();
        game.add_platforms_from_positions(vec![(1.0, 3.0)]);
        game.start_game();

        let dt = Duration::from_millis(16);
        game.next(dt);

        let player = get_player_entity(&game);
        let pos = player.components.position.as_ref().unwrap();
        let vel = player.components.velocity.as_ref().unwrap();

        assert!(
            pos.y > 1.0 && pos.y < 3.0,
            "position should increase and not snap to platform; got pos.y = {}",
            pos.y
        );
        assert!(
            vel.y > 0.0 && vel.y < 50.0,
            "velocity should remain positive but reduced by gravity; got vel.y = {}",
            vel.y
        );
    }

    #[test]
    fn bounce_when_falling_onto_platform() {
        let mut game = Game::new(10.0, 20.0, Some(42));
        game.initialise_player();
        game.add_platforms_from_positions(vec![(1.0, 5.0)]);
        game.start_game();

        let dt = Duration::from_millis(16);
        let mut bounced = false;
        let mut prev_vy = 50.0;
        for _ in 0..2000 {
            game.next(dt);
            let player = get_player_entity(&game);
            let pos = player.components.position.as_ref().unwrap();
            let vel = player.components.velocity.as_ref().unwrap();
            if prev_vy < 0.0 && (vel.y - 50.0).abs() < 1e-3 && pos.y >= 5.0 - 0.25 {
                bounced = true;
                break;
            }
            prev_vy = vel.y;
        }
        assert!(bounced, "expected to bounce on the platform when falling");
    }
}
