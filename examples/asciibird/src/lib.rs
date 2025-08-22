pub mod game;
pub mod game_objects;
use crate::game::{default_game, SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::runtime::WasmGameApi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game_api() -> WasmGameApi {
    let game = default_game();
    let asciibird_pointer = Box::new(game);
    let web_runtime = hewn::runtime::WebRuntime::new(
        asciibird_pointer,
        hewn::display::BaseDisplay {
            renderer: Box::new(hewn::display::WebRenderer::new(SCREEN_HEIGHT, SCREEN_WIDTH)),
            view_cursor: hewn::game_object::Coordinate { x: 0, y: 0 },
            cursor_strategy: Box::new(hewn::display::cursor::FollowPlayerXCursorStrategy::new()),
        },
    );
    hewn::runtime::new_wasm_game_api(web_runtime)
}
