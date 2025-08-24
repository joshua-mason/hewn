pub mod game;
use crate::game::create_game;
use hewn::runtime::WasmGameApi;
use wasm_bindgen::prelude::*;

pub const SCREEN_WIDTH: u16 = 50;
pub const SCREEN_HEIGHT: u16 = 30;

#[wasm_bindgen]
pub fn new_game_api() -> WasmGameApi {
    let game = create_game();
    let asciibird_pointer = Box::new(game);
    let web_runtime = hewn::runtime::WebRuntime::new(
        asciibird_pointer,
        hewn::view::View {
            renderer: Box::new(hewn::view::WebRenderer::new(SCREEN_HEIGHT, SCREEN_WIDTH)),
            view_cursor: hewn::view::ViewCoordinate { x: 0, y: 0 },
            cursor_strategy: Box::new(hewn::view::cursor::FollowPlayerXCursorStrategy::new()),
        },
    );
    hewn::runtime::new_wasm_game_api(web_runtime)
}
