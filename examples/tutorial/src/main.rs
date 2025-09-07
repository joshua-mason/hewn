use cgmath;
use hewn::ecs::{Components, EntityId, PositionComponent, RenderComponent, SizeComponent};
use hewn::wgpu::runtime::WindowRuntime;
use hewn::{ecs::ECS, runtime::GameHandler};
use hewn::{ecs::VelocityComponent, runtime::Key};
use std::time::Duration;

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
        let velocity = self
            .ecs
            .get_entity_by_id_mut(self.player_id)
            .and_then(|player| player.components.velocity.as_mut());
        if let Some(velocity) = velocity {
            if self.game_controller.is_up_pressed {
                velocity.y = 2.0;
            } else if self.game_controller.is_down_pressed {
                velocity.y = -2.0;
            } else {
                velocity.y = 0.0;
            }

            if self.game_controller.is_left_pressed {
                velocity.x = -2.0;
            } else if self.game_controller.is_right_pressed {
                velocity.x = 2.0;
            } else {
                velocity.x = 0.0;
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
                break; // Stop after first collision
            }
        }

        self.ecs.step(dt);
    }

    fn handle_event(&mut self, event: hewn::runtime::RuntimeEvent) -> bool {
        if let hewn::runtime::RuntimeEvent::Key(hewn::runtime::KeyEvent { key, pressed }) = event {
            self.game_controller.handle_key(key, pressed)
        } else {
            false
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
