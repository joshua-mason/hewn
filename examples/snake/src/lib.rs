pub mod game;

use crate::game::default_game;
use hewn::runtime::WasmGameApi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game_api() -> WasmGameApi {
    let width: u16 = 30;
    let height: u16 = 25;
    let game = default_game();
    let snake_pointer = Box::new(game);
    let web_runtime = hewn::runtime::WebRuntime::new(
        snake_pointer,
        hewn::display::BaseDisplay {
            renderer: Box::new(hewn::display::WebRenderer::new(height, width)),
            view_cursor: hewn::game_object::Coordinate { x: 0, y: 0 },
            cursor_strategy: Box::new(hewn::display::cursor::StaticCursorStrategy::new()),
        },
    );
    hewn::runtime::new_wasm_game_api(web_runtime)
}
