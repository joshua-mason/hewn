use cgmath::{self, InnerSpace};
use hewn::ecs::{Components, EntityId, PositionComponent, RenderComponent, SizeComponent};
use hewn::runtime::{MouseEvent, MouseLocation, RuntimeEvent};
use hewn::wgpu::runtime::WindowRuntime;
use hewn::{ecs::ECS, runtime::GameHandler};
use hewn::{ecs::VelocityComponent, runtime::Key};
use std::time::Duration;

pub struct GameController {
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    mouse_event_queue: Vec<MouseEvent>,
    mouse_location: MouseLocation,
}

impl GameController {
    pub(crate) fn new() -> Self {
        Self {
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            mouse_event_queue: Vec::new(),
            mouse_location: MouseLocation { x: 0.0, y: 0.0 },
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

    pub fn handle_mouse(&mut self, mouse: MouseEvent) -> bool {
        self.mouse_event_queue.push(mouse);
        true
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
        for mouse_event in self.game_controller.mouse_event_queue.drain(..).into_iter() {
            match mouse_event {
                MouseEvent::LeftClick => {
                    let player_entity = self.ecs.get_entity_by_id_mut(self.player_id);
                    let Some(player_entity) = player_entity else {
                        return;
                    };

                    let mouse_x = self.game_controller.mouse_location.x;
                    let player_x = player_entity.components.position.as_mut().unwrap().x;
                    let mouse_y = self.game_controller.mouse_location.y;
                    let player_y = player_entity.components.position.as_mut().unwrap().y;
                    let d_pos = cgmath::vec2(mouse_x - player_x, mouse_y - player_y);
                    let normalised_d_pos = d_pos.normalize();
                    player_entity.components.velocity.as_mut().unwrap().x = normalised_d_pos.x;
                    player_entity.components.velocity.as_mut().unwrap().y = normalised_d_pos.y;
                    let _ = player_entity;
                    self.ecs.add_entity_from_components(Components {
                        position: Some((mouse_x, mouse_y).into()),
                        velocity: None,
                        render: Some(RenderComponent {
                            ascii_character: '?',
                            rgb: (0.1, 0.1, 0.1).into(),
                        }),
                        size: Some(SizeComponent { x: 0.1, y: 0.1 }),
                        camera_follow: None,
                    });
                }
                MouseEvent::CursorMoved(location) => {
                    self.game_controller.mouse_location = location;
                }
                _ => {}
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
                break;
            }
        }

        self.ecs.step(dt);
    }

    fn handle_event(&mut self, event: RuntimeEvent) -> bool {
        match event {
            RuntimeEvent::Key(key_event) => self
                .game_controller
                .handle_key(key_event.key, key_event.pressed),
            RuntimeEvent::Mouse(mouse_event) => self.game_controller.handle_mouse(mouse_event),
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
