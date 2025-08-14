pub mod control;
pub mod display;
pub mod game;
pub mod game_object;
pub mod io;

pub use self::display::*;
pub use self::game::BaseGame;
pub use self::game::Entities;
pub use self::game_object::utils::collision_pass;
pub use self::game_object::utils::try_get_concrete_type;
pub use self::game_object::utils::try_get_mut_concrete_type;
pub use self::game_object::GameObject;
#[cfg(not(target_arch = "wasm32"))]
pub use self::io::initialize_terminal;
