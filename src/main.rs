use hewn::game::GameHandler;
use hewn::runtime::{initialize_terminal_io, TerminalRuntime, WindowRuntime};
use hewn::view::cursor::FollowPlayerYCursorStrategy;
use hewn::view::{ScreenDimensions, TerminalRenderer, View, ViewCoordinate};

const SCREEN_HEIGHT: u16 = 20;
const SCREEN_WIDTH: u16 = 50;

fn main() {
    let mut game = game::MinimalGame::new();
    game.start_game();
    let mut runtime = WindowRuntime::new();
    let _ = runtime.start(&mut game);

    let mut game = game::MinimalGame::new();
    let mut runtime = TerminalRuntime::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    runtime.start(&mut game);
}

mod game {

    pub struct GameController {
        speed: f32,
        is_up_pressed: bool,
        is_down_pressed: bool,
        is_left_pressed: bool,
        is_right_pressed: bool,
    }

    impl GameController {
        pub(crate) fn new(speed: f32) -> Self {
            Self {
                speed,
                is_up_pressed: false,
                is_down_pressed: false,
                is_left_pressed: false,
                is_right_pressed: false,
            }
        }

        pub(crate) fn handle_key(&mut self, key: Key, is_pressed: bool) -> bool {
            // Log key events for debugging
            println!(
                "[DEBUG] handle_key: key={:?} is_pressed={}",
                key, is_pressed
            );
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
        game::GameHandler,
        runtime::Key,
    };
    use winit::keyboard::KeyCode;

    pub struct MinimalGame {
        started: bool,
        pub ecs: ecs::ECS,
        pub player_entity_id: EntityId,

        pub game_controller: GameController,
        last_player_position: Option<(u16, u16)>,
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

            // Get initial player position for tracking
            let last_player_position = ecs
                .get_entity_by_id(player_entity_id)
                .and_then(|e| e.components.position.as_ref().map(|p| (p.x, p.y)));

            MinimalGame {
                started: false,
                ecs,
                player_entity_id,
                game_controller: GameController::new(1.0),
                last_player_position,
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
            if let Some(key) = key {
                println!("[DEBUG] update_player_velocity: key={:?}", key);
            }

            let player_entity = self.ecs.get_entity_by_id_mut(self.player_entity_id);
            let Some(player_entity) = player_entity else {
                println!("[DEBUG] update_player_velocity: player entity not found");
                return;
            };
            let Some(velocity) = &mut player_entity.components.velocity else {
                println!("[DEBUG] update_player_velocity: velocity component not found");
                return;
            };
            let Some(key) = &key else {
                velocity.x = 0;
                velocity.y = 0;
                // println!("[DEBUG] update_player_velocity: No key pressed, velocity set to (0,0)");
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
            println!(
                "[DEBUG] update_player_velocity: key={:?} velocity set to ({}, {})",
                key, velocity.x, velocity.y
            );
        }

        fn fix_player_velocity_to_zero(&mut self) {
            let player_entity = self.ecs.get_entity_by_id_mut(self.player_entity_id);
            let Some(player_entity) = player_entity else {
                println!("[DEBUG] fix_player_velocity_to_zero: player entity not found");
                return;
            };
            let Some(velocity) = &mut player_entity.components.velocity else {
                println!("[DEBUG] fix_player_velocity_to_zero: velocity component not found");
                return;
            };
            velocity.x = 0;
            velocity.y = 0;
        }
    }

    impl GameHandler for MinimalGame {
        fn start_game(&mut self) {
            self.started = true;
            println!("[DEBUG] Game started");
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
                println!(
                    "[DEBUG] Collision detected for player entity {:?}. Stopping movement.",
                    self.player_entity_id
                );
                self.fix_player_velocity_to_zero();
            }
            self.ecs.step();

            // After step, check if position changed
            let new_position = self
                .ecs
                .get_entity_by_id(self.player_entity_id)
                .and_then(|e| e.components.position.as_ref().map(|p| (p.x, p.y)));

            if let (Some(prev), Some(new)) = (prev_position, new_position) {
                if prev != new {
                    println!(
                        "[DEBUG] Player entity {:?} moved from ({}, {}) to ({}, {})",
                        self.player_entity_id, prev.0, prev.1, new.0, new.1
                    );
                }
            }
        }

        fn ecs(&self) -> &ECS {
            &self.ecs
        }

        fn debug_str(&self) -> Option<String> {
            let Some(player_entity) = self.ecs.get_entity_by_id(self.player_entity_id) else {
                return None;
            };
            let Some(position) = &player_entity.components.position else {
                return None;
            };

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
            let result = self.game_controller.handle_key(key, pressed);
            if result {
                println!(
                    "[DEBUG] handle_key: Player entity {:?} key={:?} pressed={}",
                    self.player_entity_id, key, pressed
                );
                // Also print current velocity and position for debugging
                if let Some(player_entity) = self.ecs.get_entity_by_id(self.player_entity_id) {
                    if let Some(velocity) = &player_entity.components.velocity {
                        println!("[DEBUG] Player velocity: ({}, {})", velocity.x, velocity.y);
                    }
                    if let Some(position) = &player_entity.components.position {
                        println!("[DEBUG] Player position: ({}, {})", position.x, position.y);
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game;
    use hewn::{game::GameHandler, runtime::Key};
    use winit::keyboard::KeyCode;

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
