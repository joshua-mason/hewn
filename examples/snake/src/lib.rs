pub mod game;

use crate::game::create_game;
use hewn::runtime::WasmGameApi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game_api(width: u16, height: u16, seed: Option<u64>) -> WasmGameApi {
    let game = create_game(width, height, seed);
    let snake_pointer = Box::new(game);
    let web_runtime = hewn::runtime::WebRuntime::new(
        snake_pointer,
        hewn::view::View {
            renderer: Box::new(hewn::view::WebRenderer::new(height, width)),
            // Perhaps both or one of view cursor + strategy should be contained in the camera component
            view_cursor: hewn::view::ViewCoordinate { x: 0, y: 0 },
            cursor_strategy: Box::new(hewn::view::cursor::StaticCursorStrategy::new()),
        },
    );
    hewn::runtime::new_wasm_game_api(web_runtime)
}

mod test {

    #[test]
    fn test_snake_body_collision() {
        let mut game = crate::new_game_api(5, 5, Some(42));
        let view = game.render();
        println!("{}", view);
        assert_eq!(view.chars().nth(0).unwrap(), '#');
    }
}
