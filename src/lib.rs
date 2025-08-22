//! # Hewn
//!
//! **Status:** Alpha – experimental crate for educational purposes.
//!
//! Hewn is a crate for making games, with support for terminal and web runtimes.
//!
//! Hewn aims to be a simple and flexible game engine, with a focus on readability and
//! maintainability.
//!
//! For more information, see the [README](https://github.com/joshua-mason/hewn).

mod engine;

pub use engine::game;
pub use engine::game_object;
pub use engine::runtime;
pub use engine::view;
