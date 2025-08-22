pub mod game;

use crate::game::default;
use hewn::WasmKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    web_runtime: hewn::runtime::WebRuntime,
}

#[wasm_bindgen]
impl Game {
    pub fn new_snake() -> Game {
        let width: u16 = 30;
        let height: u16 = 25;
        let game = default();
        let snake_pointer = Box::new(game);

        let web_runtime = hewn::runtime::WebRuntime::new(
            snake_pointer,
            hewn::display::BaseDisplay {
                renderer: Box::new(hewn::display::WebRenderer::new(height, width)),
                view_cursor: hewn::game_object::Coordinate { x: 0, y: 0 },
                // this depends on the game
                cursor_strategy: Box::new(hewn::display::cursor::StaticCursorStrategy::new()),
            },
        );
        Game { web_runtime }
    }

    pub fn start(&mut self) {
        self.web_runtime.start();
    }

    pub fn tick(&mut self, key: Option<WasmKey>) {
        self.web_runtime.tick(map_wasm_key(key));
    }

    pub fn render(&mut self) -> String {
        self.web_runtime.render()
    }
}

fn map_wasm_key(k: Option<WasmKey>) -> Option<hewn::game::Key> {
    if k.is_none() {
        return None;
    }
    let k = k.unwrap();
    match k {
        WasmKey::Left => Some(hewn::game::Key::Left),
        WasmKey::Right => Some(hewn::game::Key::Right),
        WasmKey::Up => Some(hewn::game::Key::Up),
        WasmKey::Down => Some(hewn::game::Key::Down),
        WasmKey::Space => Some(hewn::game::Key::Space),
        WasmKey::Escape => Some(hewn::game::Key::Escape),
        _ => None,
    }
}
