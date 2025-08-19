//! # Hewn
//!
//! **Status:** Alpha – This crate is an experimental game engine for learning.
//!
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
//! *

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

// /// TODO: Add documentation for WasmKey enum
// /// Represents keyboard inputs that can be sent from JavaScript to the WASM game
#[wasm_bindgen]
pub enum WasmKey {
    /// TODO: Document Left key
    Left,
    /// TODO: Document Right key
    Right,
    /// TODO: Document Up key
    Up,
    /// TODO: Document Down key
    Down,
    /// TODO: Document Space key
    Space,
    /// TODO: Document Escape key
    Escape,
}
