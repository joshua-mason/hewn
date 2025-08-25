pub mod game;
use crate::game::default_game;
use crate::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::runtime::WasmGameApi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game_api() -> WasmGameApi {
    let game = default_game();
    let asciijump_pointer = Box::new(game);
    let web_runtime = hewn::runtime::WebRuntime::new(
        asciijump_pointer,
        hewn::view::View {
            renderer: Box::new(hewn::view::WebRenderer::new(SCREEN_HEIGHT, SCREEN_WIDTH)),
            view_cursor: hewn::view::ViewCoordinate { x: 0, y: 0 },
            cursor_strategy: Box::new(hewn::view::cursor::FollowPlayerYCursorStrategy::new()),
        },
    );
    hewn::runtime::new_wasm_game_api(web_runtime)
}
