use hewn::runtime::GameHandler;
use hewn::terminal::runtime::TerminalRuntime;

const SCREEN_HEIGHT: u16 = 20;
const SCREEN_WIDTH: u16 = 50;

fn main() {
    let mut game = game::MinimalGame::new();
    game.start_game();
    let mut runtime = hewn::wgpu::runtime::WindowRuntime::new();
    let _ = runtime.start(&mut game);

    let mut game = game::MinimalGame::new();
    let mut runtime = TerminalRuntime::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    runtime.start(&mut game);
}

mod game {

    pub struct GameController {
        is_up_pressed: bool,
        is_down_pressed: bool,
        is_left_pressed: bool,
        is_right_pressed: bool,
    }

    impl GameController {
        pub(crate) fn new() -> Self {
            Self {
                is_up_pressed: false,
                is_down_pressed: false,
                is_left_pressed: false,
                is_right_pressed: false,
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
    }

    use hewn::{
        ecs::{
            self, CameraFollow, Components, EntityId, PositionComponent, RenderComponent,
            SizeComponent, VelocityComponent, ECS,
        },
        runtime::{GameHandler, Key},
    };

    pub struct MinimalGame {
        started: bool,
        pub ecs: ecs::ECS,
        pub player_entity_id: EntityId,
        pub game_controller: GameController,
    }

    impl MinimalGame {
        pub fn new() -> MinimalGame {
            let mut ecs = ecs::ECS::new();
            // Add player object
            let player_entity_id = ecs.add_entity_from_components(Components {
                position: Some(PositionComponent { x: 5, y: 5 }),
                velocity: Some(VelocityComponent { x: 0, y: 0 }),
                render: Some(RenderComponent {
                    ascii_character: 'O',
                }),
                size: Some(SizeComponent { x: 2, y: 1 }),
                camera_follow: Some(CameraFollow {}),
            });
            // Add another object as a wall
            ecs.add_entity_from_components(Components {
                position: Some(PositionComponent { x: 5, y: 6 }),
                velocity: Some(VelocityComponent { x: 0, y: 0 }),
                render: Some(RenderComponent {
                    ascii_character: '#',
                }),
                size: Some(SizeComponent { x: 2, y: 1 }),
                camera_follow: None,
            });

            MinimalGame {
                started: false,
                ecs,
                player_entity_id,
                game_controller: GameController::new(),
            }
        }

        fn update_player_velocity(&mut self) {
            let key = if self.game_controller.is_up_pressed {
                Some(Key::Up)
            } else if self.game_controller.is_down_pressed {
                Some(Key::Down)
            } else if self.game_controller.is_left_pressed {
                Some(Key::Left)
            } else if self.game_controller.is_right_pressed {
                Some(Key::Right)
            } else {
                None
            };

            let player_entity = self.ecs.get_entity_by_id_mut(self.player_entity_id);
            let Some(player_entity) = player_entity else {
                return;
            };
            let Some(velocity) = &mut player_entity.components.velocity else {
                return;
            };
            let Some(key) = &key else {
                velocity.x = 0;
                velocity.y = 0;
                return;
            };

            match key {
                Key::Left => {
                    velocity.x = -1;
                    velocity.y = 0;
                }
                Key::Right => {
                    velocity.x = 1;
                    velocity.y = 0;
                }
                Key::Up => {
                    velocity.x = 0;
                    velocity.y = 1;
                }
                Key::Down => {
                    velocity.x = 0;
                    velocity.y = -1;
                }
                _ => {}
            }
        }

        fn fix_player_velocity_to_zero(&mut self) {
            let player_entity = self.ecs.get_entity_by_id_mut(self.player_entity_id);
            let Some(player_entity) = player_entity else {
                return;
            };
            let Some(velocity) = &mut player_entity.components.velocity else {
                return;
            };
            velocity.x = 0;
            velocity.y = 0;
        }
    }

    impl GameHandler for MinimalGame {
        fn start_game(&mut self) {
            self.started = true;
        }

        fn next(&mut self) {
            if !self.started {
                return;
            }

            // Track previous position for debug
            let prev_position = self
                .ecs
                .get_entity_by_id(self.player_entity_id)
                .and_then(|e| e.components.position.as_ref().map(|p| (p.x, p.y)));

            self.update_player_velocity();
            let collisions = self.ecs.collision_pass();
            if collisions
                .iter()
                .flatten()
                .any(|entity_id| entity_id == &self.player_entity_id)
            {
                self.fix_player_velocity_to_zero();
            }
            self.ecs.step();
        }

        fn ecs(&self) -> &ECS {
            &self.ecs
        }

        fn debug_str(&self) -> Option<String> {
            let player_entity = self.ecs.get_entity_by_id(self.player_entity_id)?;
            let position = &player_entity.components.position?;

            let start_game_str = if self.started {
                "Started"
            } else {
                "Hit Space to Start"
            };
            Some(format!(
                "Player @ ({}, {}) {}",
                position.x, position.y, start_game_str
            ))
        }

        fn handle_key(&mut self, key: Key, pressed: bool) -> bool {
            self.game_controller.handle_key(key, pressed)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game;
    use hewn::runtime::{GameHandler, Key};

    #[test]
    fn test_player_move() {
        let mut game = game::MinimalGame::new();
        let player = game.ecs.get_entity_by_id(game.player_entity_id);
        assert!(player.is_some());

        game.start_game();
        game.handle_key(Key::Down, true);
        game.next();

        let player = game.ecs.get_entity_by_id(game.player_entity_id);
        assert!(player.is_some());
        let Some(player_entity) = player else {
            panic!("Player entity not set")
        };
        let Some(position) = &player_entity.components.position else {
            panic!("Position component not set")
        };
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 4);
    }

    #[test]
    fn test_player_hit_wall() {
        let mut game = game::MinimalGame::new();
        let player = game.ecs.get_entity_by_id(game.player_entity_id);
        assert!(player.is_some());

        game.start_game();
        game.handle_key(Key::Up, true);
        game.next();

        let player = game.ecs.get_entity_by_id(game.player_entity_id);
        assert!(player.is_some());
        let Some(player_entity) = player else {
            panic!("Player entity not set")
        };
        let Some(position) = &player_entity.components.position else {
            panic!("Position component not set")
        };
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 5);
    }
}
