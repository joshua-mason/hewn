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
> **Complete tutorial code** is available in `examples/tutorial/`. The following tutorial builds up from the simplest possible game.

### Step 1: Hello World

Let's start with the simplest possible game - showing debug text in the terminal.

First, let's create the basic game structure:

```rust
use hewn::{
    runtime::GameHandler, // 1.
    ecs::ECS, // 2.
};

struct HelloGame {
    ecs: ECS, // 3.
}

impl HelloGame {
    fn new() -> Self {
        Self { ecs: ECS::new() }
    }
}

impl GameHandler for HelloGame { // 4.
    fn start_game(&mut self) {}
    fn next(&mut self) {}
    fn handle_key(&mut self, _key: hewn::runtime::Key, _pressed: bool) -> bool { true }
    fn ecs(&self) -> &ECS { &self.ecs }
    
    fn debug_str(&self) -> Option<String> {
        Some("Hello Hewn! Press Q to exit.".to_string()) // 6.
    }
}
```

**What changed:**

1. Import `GameHandler` trait - the core interface all Hewn games implement
2. Import `ECS` - the Entity Component System that manages game objects
3. `HelloGame` struct holds our game state (just an ECS for now)
4. Implement `GameHandler` trait with required methods
5. `debug_str()` returns text that appears at the bottom of the terminal

Next, let's run our game:

```rust
use hewn::terminal::runtime::TerminalRuntime;

// ..

fn main() {
    let mut game = HelloGame::new(); // 1.
    let mut runtime = TerminalRuntime::new(20, 20); // 2.
    runtime.start(&mut game); // 3.
}
```

1. Create an instance of our game
2. Create a terminal runtime with 20×20 character display. We will get to the window runtime later.
3. Start the game loop - this runs until the user presses 'Q'

This creates a minimal game that shows "Hello Hewn! Press Q to exit." at the bottom of your terminal. All Hewn games implement the `GameHandler` trait and need an ECS (Entity Component System) to manage game objects.

> [!TIP]
> Run this with `cargo run` and you'll see a field of `.` characters representing empty space, with your debug text at the bottom. We're about to add a character that moves around this world!

### Step 2: Add a Visible Character

Now let's add a character that appears on screen. This is where the ECS comes in - we'll create an entity with position and rendering components.

First, let's create the player entity:

```rust
// ..
use hewn::{
    ecs::{ECS, EntityId, Components, PositionComponent, RenderComponent, SizeComponent}, // NEW!
    runtime::GameHandler,
    terminal::runtime::TerminalRuntime,
};

struct HelloGame {
    ecs: ECS,
    player_id: EntityId, // 1.
}

impl HelloGame {
    fn new() -> Self {
        let mut ecs = ECS::new();
        
        let player_id = ecs.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 5, y: 5 }), // 2.
            render: Some(RenderComponent { ascii_character: '@' }), // 3.
            velocity: None,
            size: Some(SizeComponent { x: 1, y: 1 }), // 4.
            camera_follow: None,
        });
        
        Self { ecs, player_id }
    }
}
// ..
```

**What changed:**

1. Added `player_id` field to store a reference to our character entity
2. Player positioned at coordinates (5, 5) in the game world  
3. `RenderComponent` makes the entity appear as `@` character on screen
4. `SizeComponent` defines the entity's collision box (1×1 unit)

Next, let's update the game loop and debug display:

```rust
// ..
impl GameHandler for HelloGame {
    // ..
    fn next(&mut self) {
        self.ecs.step(); // 1.
    }
    // ..
    
    fn debug_str(&self) -> Option<String> {
        let player = self.ecs.get_entity_by_id(self.player_id)?; // 2.
        let pos = player.components.position.as_ref()?;
        Some(format!("Player @ ({}, {})", pos.x, pos.y)) // 3.
    }
}
// ..
```

**What changed:**

1. `ecs.step()` updates all entities each frame (position, rendering, etc.)
2. Look up the player entity by its ID to access its components
3. Debug text now shows the player's live position coordinates

Now you'll see an `@` character in your terminal surrounded by `.` tiles! The debug text at the bottom shows its exact position. 

### Step 3: Add Movement

Let's make our character respond to arrow keys using a controller pattern.

First, let's create a controller to track key states:

```rust
// ..
use hewn::{
    runtime::Key, // NEW!
    ecs::VelocityComponent, // NEW!
};

// Add a controller to track key states
pub struct GameController { // 1.
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
        match key { // 2.
            Key::Up => { self.is_up_pressed = is_pressed; true }
            Key::Down => { self.is_down_pressed = is_pressed; true }
            Key::Left => { self.is_left_pressed = is_pressed; true }
            Key::Right => { self.is_right_pressed = is_pressed; true }
            _ => false,
        }
    }
}
// ..
```

**What changed:**

1. `GameController` struct tracks the current state of arrow keys (pressed/not pressed)
2. `handle_key()` updates the key states when keys are pressed or released

Next, let's integrate the controller into our game and add velocity:

```rust
// ..
struct HelloGame {
    ecs: ECS,
    player_id: EntityId,
    game_controller: GameController, // 1.
}

impl HelloGame {
    fn new() -> Self {
        let mut ecs = ECS::new();
        
        let player_id = ecs.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 5, y: 5 }), 
            render: Some(RenderComponent { ascii_character: '@' }),
            velocity: Some(VelocityComponent { x: 0, y: 0 }), // 2.
            size: Some(SizeComponent { x: 2, y: 1 }), // 3.
            camera_follow: None,
        });
        
        Self { 
            ecs, 
            player_id,
            game_controller: GameController::new(), // 4.
        }
    }
}

impl GameHandler for HelloGame {
    // ..
    fn next(&mut self) {
        // Update player velocity based on controller state
        let velocity = self.ecs.get_entity_by_id_mut(self.player_id)
            .and_then(|player| player.components.velocity.as_mut());
        if let Some(velocity) = velocity {
            if self.game_controller.is_up_pressed { // 5.
                velocity.y = 1;
            } else if self.game_controller.is_down_pressed {
                velocity.y = -1;
            } else {
                velocity.y = 0;
            }

            if self.game_controller.is_left_pressed {
                velocity.x = -1;
            } else if self.game_controller.is_right_pressed {
                velocity.x = 1;
            } else {
                velocity.x = 0;
            }
        }
        
        self.ecs.step(); // 6.
    }
    
    fn handle_key(&mut self, key: Key, pressed: bool) -> bool {
        self.game_controller.handle_key(key, pressed) // 7.
    }
}
// ..
```

**What changed:**

1. Added `game_controller` field to track input state
2. Player now has a `VelocityComponent` - the ECS automatically moves entities with velocity
3. Player size is `2x1` so it appears wider in the terminal  
4. Initialize the controller in the constructor
5. `next()` method reads controller state and updates player velocity accordingly
6. `ecs.step()` applies the velocity to move the player
7. `handle_key()` delegates to the controller for clean separation of concerns

Your `@` character now responds to arrow keys! Try moving around and watch the debug text update with your position. Now let's see the same game running in a desktop window...

### Step 4: Same Game, Desktop Window

Now we've built our game, it's possible to run in our `WindowRuntime`. Without changing our game, we use the `wgpu` runtime:

```rust
// ..
use hewn::wgpu::runtime::WindowRuntime; // NEW!

// ..

fn main() {
    let mut game = HelloGame::new(); // Same game!
    let mut runtime = WindowRuntime::new(); // 1.
    let _ = runtime.start(&mut game);
}
```

**What changed:**

1. Swap `TerminalRuntime` for `WindowRuntime` - that's literally it!

Your `@` character now renders as a colored square in a desktop window.

### Step 5: Add Collision Detection

Let's add a wall that blocks the player's movement to make it feel like a real game.

First, let's add a wall entity:

```rust
// ..
impl HelloGame {
    fn new() -> Self {
        let mut ecs = ECS::new();
        
        // .. 

        // Add a wall
        ecs.add_entity_from_components(Components {
            position: Some(PositionComponent { x: 8, y: 5 }), // 1.
            render: Some(RenderComponent { ascii_character: '#' }), // 2.
            velocity: None, // 3.
            size: Some(SizeComponent { x: 2, y: 1 }), // 4.
            camera_follow: None,
        });
        // ..
    }
}
// ..
```

**What changed:**

1. Wall positioned at (8, 5) - to the right of the player starting position
2. Wall renders as `#` character on screen
3. Wall has no velocity (it doesn't move) - note that this could also be set to `VelocityComponent { x: 0, y: 0 }`.
4. Wall has 2×1 size, so it appears as `##` (2 units wide)

If you run now, you'll see the player move through the wall. Next, let's add collision detection to the game loop:

```rust
// ..
impl GameHandler for HelloGame {
    // ..
    fn next(&mut self) {
        
        // .. Velocity update logic from Step 3 ..  

        // Check for collisions BEFORE moving entities
        let collisions = self.ecs.collision_pass(); // 1.
        for [a, b] in collisions.into_iter() { // 2.
            if a == self.player_id || b == self.player_id {
                let player_entity = self.ecs.get_entity_by_id_mut(self.player_id);
                let Some(player_entity) = player_entity else { return; };
                let Some(velocity) = &mut player_entity.components.velocity else { return; };
                velocity.x = 0; // 3.
                velocity.y = 0;
                break; // Stop after first collision
            }
        }
        
        self.ecs.step(); // 4. Move entities AFTER collision check
    }
    // ..
}
```

**What changed:**

1. `collision_pass()` returns pairs of entities that are colliding
2. **Important**: Iterate over collision pairs `[a, b]` - don't use `.flatten()` which loses pairing info
3. When collision detected, immediately stop the player by resetting velocity to `(0, 0)`
4. **Critical**: Call `ecs.step()` AFTER collision detection to apply the movement

> [!IMPORTANT]
> The order of operations in `next()` matters! Update velocity → Check collisions → Apply movement. This prevents the player from "tunneling" through walls.

Now you'll see a `##` wall that blocks your `@` character's movement! Try moving right into it.

🎉 Congratulations! You’ve built a simple game with movement and collision using Hewn. Explore, experiment, and have fun making your own games! Check the examples or docs for more advanced features.


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
