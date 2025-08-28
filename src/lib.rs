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

pub mod render;
pub mod runtime;
pub mod texture;

pub use engine::ecs;
pub use engine::view;
