//! TODO: Add module documentation for engine
//! Core game engine providing rendering, input handling, and game object management

pub mod display;
pub mod game;
pub mod game_object;
pub mod runtime;

pub use self::display::*;
pub use self::game_object::GameObject;
#[cfg(not(target_arch = "wasm32"))]
pub use self::runtime::initialize_terminal_io;
