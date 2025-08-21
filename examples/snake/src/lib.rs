pub mod game;

use crate::game::default;
use hewn::WasmKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    web_control: hewn::control::WebControl,
}

#[wasm_bindgen]
impl Game {
    pub fn new_snake() -> Game {
        let width: u16 = 30;
        let height: u16 = 25;
        let game = default();
        let snake_pointer = Box::new(game);

        let web_control = hewn::control::WebControl::new(
            snake_pointer,
            hewn::display::BaseDisplay {
                renderer: Box::new(hewn::display::WebRenderer::new(height, width)),
                view_cursor: hewn::game_object::Coordinate { x: 0, y: 0 },
                // this depends on the game
                cursor_strategy: Box::new(hewn::display::cursor::StaticCursorStrategy::new()),
            },
        );
        Game { web_control }
    }

    pub fn start(&mut self) {
        self.web_control.start();
    }

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

    pub fn tick(&mut self) {
        self.web_control.tick();
    }

    pub fn render(&mut self) -> String {
        self.web_control.render()
    }
}
