# Hewn

Hewn is a primitive Rust game engine for learning and tinkering, with Terminal and WebAssembly support.

- Crate: [crates.io/hewn](https://crates.io/crates/hewn)
- Examples: `examples/asciijump`, `examples/asciibird`, `examples/snake`

## Quick start

The following is a minimal example of a game in Hewn, allowing a player to move a character around the screen in a terminal.

```rust
use hewn::game::{Entities, GameLogic};
use hewn::game_object::{CollisionBox, Coordinate, GameObject};
use hewn::runtime::{initialize_terminal_io, Key, TerminalRuntime};
use hewn::view::cursor::StaticCursorStrategy;
use hewn::view::{TerminalRenderer, View};
use std::any::Any;

#[derive(Debug)]
struct Player {
    coords: Coordinate,
}

impl Player {
    fn new(x: usize, y: usize) -> Player {
        Player {
            coords: Coordinate { x, y },
        }
    }
}

impl GameObject for Player {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn collide(&mut self, _other: &dyn GameObject) {}

    fn display(&self) -> String {
        "@".to_owned()
    }

    fn get_collision_box(&self) -> CollisionBox {
        CollisionBox {
            x: self.coords.x..(self.coords.x + self.width()),
            y: self.coords.y..(self.coords.y + 1),
        }
    }

    fn get_coords(&self) -> &Coordinate {
        &self.coords
    }

    fn next_step(&mut self) {}

    fn priority(&self) -> u8 {
        1
    }

    fn width(&self) -> usize {
        1
    }

    fn is_player(&self) -> bool {
        true
    }
}

struct MinimalGame {
    entities: Entities,
    started: bool,
}

impl MinimalGame {
    fn new() -> MinimalGame {
        let mut entities = Entities::new();
        let mut objects: Vec<Box<dyn GameObject>> = vec![Box::new(Player::new(5, 5))];
        entities.add_game_objects(&mut objects);
        MinimalGame {
            entities,
            started: true,
        }
    }

    fn move_player(&mut self, key: Key) {
        if let Some(player) = self
            .entities
            .game_objects
            .iter_mut()
            .find(|o| o.is_player())
        {
            if let Some(p) = player.as_mut_any().downcast_mut::<Player>() {
                match key {
                    Key::Left => {
                        p.coords.x = p.coords.x.saturating_sub(1);
                    }
                    Key::Right => {
                        p.coords.x = p.coords.x.saturating_add(1);
                    }
                    Key::Up => {
                        p.coords.y = p.coords.y.saturating_add(1);
                    }
                    Key::Down => {
                        p.coords.y = p.coords.y.saturating_sub(1);
                    }
                    _ => {}
                }
            }
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
        if let Some(k) = key {
            self.move_player(k);
        }
    }

    fn entities(&self) -> &Entities {
        &self.entities
    }

    fn debug_str(&self) -> Option<String> {
        let player = self.entities.game_objects.iter().find(|o| o.is_player())?;
        let c = player.get_coords();
        Some(format!("Player @ ({}, {})", c.x, c.y))
    }
}

fn main() {
    let (stdout, stdin) = initialize_terminal_io();

    let screen_height: u16 = 20;
    let screen_width: u16 = 50;

    let mut view = View {
        view_cursor: Coordinate { x: 0, y: 0 },
        renderer: Box::new(TerminalRenderer::new(stdout, screen_height, screen_width)),
        cursor_strategy: Box::new(StaticCursorStrategy::new()),
    };

    let mut game = MinimalGame::new();
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut view);
    runtime.start();
}
```

## Examples

```bash
# Terminal
cargo run -p asciijump

# Web (serve locally)
# Install wasm-pack if you haven't already
# https://drager.github.io/wasm-pack/installer/
cd examples/asciijump
wasm-pack build --release --target web
python3 -m http.server
```
