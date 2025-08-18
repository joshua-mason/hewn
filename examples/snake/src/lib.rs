use hewn::WasmKey;
use wasm_bindgen::prelude::*;

pub mod game;

#[wasm_bindgen]
pub struct Game {
    web_control: hewn::control::WebControl,
}

#[wasm_bindgen]
impl Game {
    pub fn new_snake() -> Game {
        let width: u16 = 30;
        let height: u16 = 25;
        let web_control = hewn::control::WebControl::new(
            Box::new(crate::game::snake::Game::new(
                width as usize,
                height as usize,
            )),
            hewn::display::BaseDisplay {
                renderer: Box::new(hewn::display::WebRenderer::new(height, width)),
                view_cursor: hewn::game_object::Coordinate { x: 0, y: 0 },
                // this depends on the game
                cursor_strategy: Box::new(hewn::display::cursor::StaticCursorStrategy::new()),
            },
        );
        Game { web_control }
    }

    /// TODO: Add documentation for start method
    /// Initializes and starts the game
    pub fn start(&mut self) {
        self.web_control.start();
    }

    /// TODO: Add documentation for set_player_control_key method
    /// Sets the current player input key
    ///
    /// # Arguments
    /// TODO: Document key parameter
    pub fn set_player_control_key(&mut self, key: Option<WasmKey>) {
        fn map_wasm_key(k: WasmKey) -> hewn::game::Key {
            match k {
                WasmKey::Left => hewn::game::Key::Left,
                WasmKey::Right => hewn::game::Key::Right,
                WasmKey::Up => hewn::game::Key::Up,
                WasmKey::Down => hewn::game::Key::Down,
                WasmKey::Space => hewn::game::Key::Space,
                WasmKey::Escape => hewn::game::Key::Escape,
            }
        }
        self.web_control
            .set_player_control_key(key.map(map_wasm_key));
    }

    /// TODO: Add documentation for tick method
    /// Advances the game by one frame/step
    pub fn tick(&mut self) {
        self.web_control.tick();
    }

    /// TODO: Add documentation for render method
    /// Renders the current game state and returns it as a string
    ///
    /// # Returns
    /// TODO: Document return value format
    pub fn render(&mut self) -> String {
        self.web_control.render()
    }
}
