use cgmath;
use hewn::scene::{Components, EntityId, PositionComponent, RenderComponent, SizeComponent};
use hewn::wgpu::render::{CameraStrategy, Tilemap};
use hewn::wgpu::runtime::WindowRuntime;
use hewn::{runtime::GameHandler, scene::Scene};
use hewn::{runtime::Key, scene::VelocityComponent};
use std::time::Duration;

// Embed the tilemap at compile time
const TILEMAP_BYTES: &[u8] =
    include_bytes!("../../snake/src/assets/monochrome_tilemap_transparent_packed.png");

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
    scene: Scene,
    player_id: EntityId,
    game_controller: GameController,
}

impl HelloGame {
    fn new() -> Self {
        let mut scene = Scene::new();

        // Player entity - blue square, no sprite (solid color)
        let player_id = scene.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 5.0, y: 5.0 }),
            render: Some(RenderComponent {
                ascii_character: '@',
                rgb: cgmath::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                sprite_tile: None, // No texture - render as solid blue
            }),
            velocity: Some(VelocityComponent { x: 0.0, y: 0.0 }),
            size: Some(SizeComponent { x: 2.0, y: 1.0 }),
            camera_follow: None,
        });
        
        // Wall entity - uses tile 69 from tilemap (a tree/plant sprite)
        scene.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 8.0, y: 5.0 }),
            render: Some(RenderComponent {
                ascii_character: '#',
                rgb: cgmath::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                sprite_tile: Some(69), // Tile 69 in the 20x20 tilemap
            }),
            velocity: None,
            size: Some(SizeComponent { x: 2.0, y: 1.0 }),
            camera_follow: None,
        });

        Self {
            scene,
            player_id,
            game_controller: GameController::new(),
        }
    }
}

impl GameHandler for HelloGame {
    fn start_game(&mut self) {}

    fn next(&mut self, dt: Duration) {
        let velocity = self
            .scene
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

        let collisions = self.scene.collision_pass(dt);
        for [a, b] in collisions.into_iter() {
            if a == self.player_id || b == self.player_id {
                let player_entity = self.scene.get_entity_by_id_mut(self.player_id);
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

        self.scene.step(dt);
    }

    fn handle_key(&mut self, key: Key, pressed: bool) -> bool {
        self.game_controller.handle_key(key, pressed)
    }

    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn debug_str(&self) -> Option<String> {
        let player = self.scene.get_entity_by_id(self.player_id)?;
        let pos = player.components.position.as_ref()?;
        Some(format!("Player @ ({}, {})", pos.x, pos.y))
    }
}

fn main() {
    let mut game = HelloGame::new();
    let mut runtime = WindowRuntime::new();
    let entity_id = game.player_id;
    
    // Create tilemap configuration (20x20 tiles)
    let tilemap = Tilemap::new(TILEMAP_BYTES, 20, 20);
    
    let _ = runtime.start(
        &mut game,
        CameraStrategy::CameraFollow(entity_id),
        Some(tilemap),
    );
}
