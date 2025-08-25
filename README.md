# Hewn

Hewn is a primitive Rust game engine for learning and tinkering, with Terminal and WebAssembly support.

- Crate: [crates.io/hewn](https://crates.io/crates/hewn)
- Examples: `examples/asciijump`, `examples/asciibird`, `examples/snake`

## Quick start

The following is a minimal example of a game in Hewn, allowing a player to move a character around the screen in a terminal.

```rust
use hewn::runtime::{initialize_terminal_io, TerminalRuntime};
use hewn::view::cursor::FollowPlayerYCursorStrategy;
use hewn::view::{TerminalRenderer, View, ViewCoordinate};

const SCREEN_HEIGHT: u16 = 20;
const SCREEN_WIDTH: u16 = 50;

mod game {

    use hewn::{
        ecs::{
            self, Components, EntityId, PositionComponent, RenderComponent, SizeComponent,
            TrackComponent, VelocityComponent, ECS,
        },
        game::GameLogic,
        runtime::Key,
    };

    pub struct MinimalGame {
        started: bool,
        pub ecs: ecs::ECS,
        pub player_entity_id: EntityId,
    }

    impl MinimalGame {
        pub fn new() -> MinimalGame {
            let mut ecs = ecs::ECS::new();
            // Add player object
            let player_entity_id = ecs.add_entity_froms(Components {
                position: Some(PositionComponent { x: 5, y: 5 }),
                velocity: Some(VelocityComponent { x: 0, y: 0 }),
                render: Some(RenderComponent {
                    ascii_character: 'O',
                }),
                size: Some(SizeComponent { x: 2, y: 1 }),
                track: Some(TrackComponent {}),
            });

            // Add another object as a wall
            ecs.add_entity_froms(Components {
                position: Some(PositionComponent { x: 5, y: 6 }),
                velocity: Some(VelocityComponent { x: 0, y: 0 }),
                render: Some(RenderComponent {
                    ascii_character: '#',
                }),
                size: Some(SizeComponent { x: 2, y: 1 }),
                track: None,
            });

            MinimalGame {
                started: false,
                ecs,
                player_entity_id,
            }
        }

        fn update_player_velocity(&mut self, key: Option<Key>) {
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
    }

    impl GameLogic for MinimalGame {
        fn start_game(&mut self) {
            self.started = true;
        }

        fn next(&mut self, key: Option<Key>) {
            if !self.started {
                return;
            }

            self.update_player_velocity(key);
            let collisions = self.ecs.collision_pass();
            if collisions
                .iter()
                .flatten()
                .any(|entity_id| entity_id == &self.player_entity_id)
            {
                self.update_player_velocity(None);
            }
            self.ecs.step();
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
    }
}

fn main() {
    let (stdout, stdin) = initialize_terminal_io();

    let mut view = View {
        view_cursor: ViewCoordinate { x: 0, y: 0 },
        renderer: Box::new(TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH)),
        cursor_strategy: Box::new(FollowPlayerYCursorStrategy::new()),
    };

    let mut game = game::MinimalGame::new();
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut view);
    runtime.start();
}

#[cfg(test)]
mod test {
    use hewn::{game::GameLogic, runtime::Key};
    use crate::game;

    #[test]
    fn test_player_move() {
        let mut game = game::MinimalGame::new();
        let player = game.ecs.get_entity_by_id(game.player_entity_id);
        assert!(player.is_some());

        game.start_game();
        game.next(Some(Key::Down));

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
        game.next(Some(Key::Up));

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
```

## Examples

```bash
# Terminal
cargo run -p asciijump

# Web (serve locally)
# Install wasm-pack (this helps install the correct version of wasm-bindgen 
# for the project and build for different web targets)
# https://drager.github.io/wasm-pack/installer/
cd examples/asciijump
wasm-pack build --release --target web
python3 -m http.server
```

