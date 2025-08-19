pub mod game;
pub mod game_objects;

use crate::game::{default, SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::WasmKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    web_control: hewn::control::WebControl,
}

#[wasm_bindgen]
impl Game {
    pub fn new_asciibird() -> Game {
        let game = default();

        let asciibird_pointer = Box::new(game);
        let web_control = hewn::control::WebControl::new(
            asciibird_pointer,
            hewn::display::BaseDisplay {
                renderer: Box::new(hewn::display::WebRenderer::new(SCREEN_HEIGHT, SCREEN_WIDTH)),
                view_cursor: hewn::game_object::Coordinate { x: 0, y: 0 },
                cursor_strategy: Box::new(hewn::display::cursor::FollowPlayerXCursorStrategy::new()),
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
