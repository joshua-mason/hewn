//! # Hewn
//!
//! **Status:** Alpha – This is an experimental crate for educational purposes.
//!
//! Hewn is a crate for making games in the terminal and exporting as WebAssembly.
//!
//! Hewn was created as an abstraction from an original terminal game, `asciijump`, in order
//! to explore how a basic game engine might work in a low level language.
//!
//! <!--
//! # Getting started
//!
//! We represent a game in this library with these core concepts:
//!
//! * the trait `BaseGame` represents the central controller for game logic, on the game loop, where we can access the game objects
//! * the trait `GameObject` represents the items in a game, for example players, walls, platforms.
//! * The Controller (`WebController` or `TerminalController`) orchestrate the `BaseGame` with user IO. These are have different implementations depending on the type of game. We support terminal and web based games (via wasm).
//! * The CursorStrategy which controls the player's view of the game. There are a few out-of-the-box strategies implemented in the library (`FollowPlayerYCursorStrategy`, `FollowPlayerXCursorStrategy` and `StaticCursorStrategy`)
//!
//! ## Game Objects
//!
//!
//! -->

use wasm_bindgen::prelude::*;

mod engine;

pub use engine::control;
pub use engine::control::WebControl;
pub use engine::cursor;
pub use engine::display;
pub use engine::game;
pub use engine::game_object;
#[cfg(not(target_arch = "wasm32"))]
pub use engine::initialize_terminal;
pub use engine::BaseDisplay;
#[cfg(not(target_arch = "wasm32"))]
pub use engine::TerminalRenderer;

#[wasm_bindgen]
pub enum WasmKey {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}
