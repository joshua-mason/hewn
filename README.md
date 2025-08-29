# Hewn

Hewn is a minimal Rust game engine for learning and tinkering, with support for terminal, desktop, and web platforms.

- Crate: [crates.io/hewn](https://crates.io/crates/hewn)
- Examples: `examples/asciijump`, `examples/asciibird`, `examples/snake`

## Features

> [!WARNING]
> Hewn has only been tested on macOS so far. Windows and Linux support is untested and may have issues.

- 🖥️ **Terminal** - ASCII games in your terminal with debug output
- 🖼️ **Desktop** - Hardware-accelerated graphics with WGPU  
- 🌐 **Web** - Deploy to HTML5 canvas
- 🎮 **ECS** - Entity Component System architecture
- ⚡ **Cross-Platform** - Write once, run anywhere

## Quick Start

> [!NOTE]
> **Complete examples** are available in `src/main.rs` and the `examples/` directory. The following tutorial builds up from the simplest possible game.

### Step 1: Hello World

Let's start with the simplest possible game - showing debug text in the terminal:

```rust
use hewn::{
    runtime::GameHandler,
    terminal::runtime::TerminalRuntime,
    ecs::ECS,
};

struct HelloGame {
    ecs: ECS,
}

impl GameHandler for HelloGame {
    fn start_game(&mut self) {}
    fn next(&mut self) {}
    fn handle_key(&mut self, _key: hewn::runtime::Key, _pressed: bool) -> bool { true }
    fn ecs(&self) -> &ECS { &self.ecs }
    
    fn debug_str(&self) -> Option<String> {
        Some("Hello Hewn! Press Q to exit.".to_string())
    }
}

fn main() {
    let mut game = HelloGame { ecs: ECS::new() };
    let mut runtime = TerminalRuntime::new(20, 20);
    runtime.start(&mut game);
}
```

This creates a minimal game that shows "Hello Hewn! Press Q to exit." at the bottom of your terminal. All Hewn games implement the `GameHandler` trait and need an ECS (Entity Component System) to manage game objects.

> [!TIP]
> Run this with `cargo run` and you'll see a field of `.` characters representing empty space, with your debug text at the bottom. We're about to add a character that moves around this world!

### Step 2: Add a Visible Character

Now let's add a character that appears on screen. This is where the ECS comes in - we'll create an entity with position and rendering components:

```rust
// ..
use hewn::{
    ecs::{ECS, EntityId, Components, PositionComponent, RenderComponent, SizeComponent}, // NEW!
};

struct HelloGame {
    // ..
    player_id: EntityId, // 1.
}

impl HelloGame {
    fn new() -> Self { // 2.
        let mut ecs = ECS::new();
        
        let player_id = ecs.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 5, y: 5 }), // 3.
            render: Some(RenderComponent { ascii_character: '@' }),
            velocity: None,
            size: Some(SizeComponent { x: 1, y: 1 }),
            camera_follow: None,
        });
        
        Self { ecs, player_id }
    }
}

impl GameHandler for HelloGame {
    // ..
    fn next(&mut self) {
        self.ecs.step(); // 4.
    }
    
    // ..
    fn debug_str(&self) -> Option<String> {
        let player = self.ecs.get_entity_by_id(self.player_id)?;
        let pos = player.components.position.as_ref()?;
        Some(format!("Player @ ({}, {})", pos.x, pos.y)) // 5.
    }
}

// ..


fn main() {
    let mut game = HelloGame::new();
    // ..
}

```

**What changed:**

1. We store a `player_id` to reference our character entity. We use a custom type for the ID (like EntityId) to prevent mixing up entity IDs with other integers, improving type safety and making the code clearer and less error-prone.
2. Added a constructor that creates a player entity in the ECS
3. The player has position and render components - it appears as `@` at coordinates (5, 5)
4. `ecs.step()` updates all entities each frame
5. Debug text now shows the player's current position

Now you'll see an `@` character in your terminal surrounded by `.` tiles! The debug text at the bottom shows its exact position. 

### Step 3: Add Movement

Let's make our character respond to arrow keys:

```rust
// ..
use hewn::{
    runtime::Key, // NEW!
    ecs::VelocityComponent, // NEW!
};

// ..

impl HelloGame {
    fn new() -> Self {
        // ..
        let player_id = ecs.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 5, y: 5 }), 
            render: Some(RenderComponent { ascii_character: '@' }),
            velocity: Some(VelocityComponent { x: 0, y: 0 }), // 1.
            size: Some(SizeComponent { x: 1, y: 1 }),
            camera_follow: None,
        });
        // ..
    }
}

impl GameHandler for HelloGame {
    // ..
    fn handle_key(&mut self, key: Key, pressed: bool) -> bool {
        let velocity = self
            .ecs
            .get_entity_by_id_mut(self.player_id)
            .and_then(|player| player.components.velocity.as_mut());
        if let Some(velocity) = velocity {
            match key {
                // 2.
                Key::Up => velocity.y = if pressed { 1 } else { 0 },
                Key::Down => velocity.y = if pressed { -1 } else { 0 },
                Key::Left => velocity.x = if pressed { -1 } else { 0 },
                Key::Right => velocity.x = if pressed { 1 } else { 0 },
                _ => return false,
            }
        }
        true
    }
}
// ..
```

**What changed:**

1. Player now has a `VelocityComponent` - the ECS automatically moves entities with velocity
2. Arrow keys set velocity when pressed and stop movement when released

Your `@` character now responds to arrow keys! Try moving around and watch the debug text update with your position. Now let's see the same game running in a desktop window...

### Step 4: Same Game, Desktop Window

Now we've built our game, it's possible to run in our `WindowRuntime`. Without changing our game, we use the `wgpu` runtime:

```rust
// ..
use hewn::wgpu::runtime::WindowRuntime; // NEW!

fn main() {
    let mut game = HelloGame::new(); // Same game!
    let mut runtime = WindowRuntime::new(); // 1.
    let _ = runtime.start(&mut game);
}
```

**What changed:**

1. Swap `TerminalRuntime` for `WindowRuntime` - that's literally it!

Your `@` character now renders as a colored square in a desktop window.

> [!NOTE]
> This demonstrates Hewn's flexibility - the ECS and GameHandler pattern abstract away the platform differences.



---

## Platform Support

| Platform | Runtime | Command | Use Case |
|----------|---------|---------|----------|
| **Terminal** | `TerminalRuntime` | `cargo run` | ASCII games, debugging, servers |
| **Desktop** | `WindowRuntime` | `cargo run` | Native apps, high performance |
| **Web** | WASM + Canvas | `wasm-pack build` | Browser games |

## Architecture

Hewn games implement the `GameHandler` trait:

- **`start_game()`** - Initialize your game state
- **`next()`** - Update game logic each frame  
- **`handle_key()`** - Process keyboard input
- **`ecs()`** - Access the Entity Component System
- **`debug_str()`** - Show debug info (terminal only)

The ECS manages entities with components:
- **`PositionComponent`** - Where entities are located
- **`VelocityComponent`** - How entities move  
- **`RenderComponent`** - How entities look
- **`SizeComponent`** - Entity collision bounds
- **`CameraFollow`** - Camera tracks this entity

## Examples

### Run the Built-in Examples

```bash
# Terminal snake game
cargo run -p snake

# Terminal platformer  
cargo run -p asciijump

# Terminal flying game
cargo run -p asciibird
```

### Web Deployment

```bash
# Install wasm-pack if you haven't
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build any example for web
cd examples/snake
wasm-pack build --release --target web

# Serve locally
python3 -m http.server
# Open http://localhost:8000
```

---

**Happy game making!** 🎮

## License

This project is licensed under [your license here].