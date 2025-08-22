pub mod game;
pub mod game_objects;

use crate::game::{default, SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::WasmKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    web_runtime: hewn::runtime::WebRuntime,
}

#[wasm_bindgen]
impl Game {
    pub fn new_asciibird() -> Game {
        let game = default();

        let asciibird_pointer = Box::new(game);
        let web_runtime = hewn::runtime::WebRuntime::new(
            asciibird_pointer,
            hewn::display::BaseDisplay {
                renderer: Box::new(hewn::display::WebRenderer::new(SCREEN_HEIGHT, SCREEN_WIDTH)),
                view_cursor: hewn::game_object::Coordinate { x: 0, y: 0 },
                cursor_strategy: Box::new(hewn::display::cursor::FollowPlayerXCursorStrategy::new()),
            },
        );
        Game { web_runtime }
    }

    pub fn start(&mut self) {
        self.web_runtime.start();
    }

    pub fn tick(&mut self, key: Option<WasmKey>) {
        self.web_runtime.tick(key.map(map_wasm_key));
    }

    pub fn render(&mut self) -> String {
        self.web_runtime.render()
    }
}

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
