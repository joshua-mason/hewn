pub mod game;

use crate::game::create_game;
use hewn::runtime::WasmGameApi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game_api() -> WasmGameApi {
    let width: u16 = 30;
    let height: u16 = 25;
    let game = create_game(width, height);
    let snake_pointer = Box::new(game);
    let web_runtime = hewn::runtime::WebRuntime::new(
        snake_pointer,
        // should the game actually own the display and the runtime only handle the IO?
        hewn::view::View {
            renderer: Box::new(hewn::view::WebRenderer::new(height, width)),
            view_cursor: hewn::view::ViewCoordinate { x: 0, y: 0 },
            cursor_strategy: Box::new(hewn::view::cursor::StaticCursorStrategy::new()),
        },
    );
    hewn::runtime::new_wasm_game_api(web_runtime)
}

mod test {

    #[test]
    fn test_snake_body_collision() {
        let mut game = crate::new_game_api();
        let view = game.render();
        println!("{}", view);
        assert_eq!(view.chars().nth(0).unwrap(), '#');
    }
}
