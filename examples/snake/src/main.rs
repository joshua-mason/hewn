mod game;
use crate::game::create_game;
#[cfg(not(target_arch = "wasm32"))]
use hewn::{
    runtime::GameHandler,
    terminal::{render::cursor::StaticCursorStrategy, runtime::TerminalRuntime},
};

const DEFAULT_WIDTH: u16 = 25;
const DEFAULT_HEIGHT: u16 = 25;

fn main() {
    // Default to terminal to keep `cargo run -p snake` predictable.
    // Use `--wgpu` to run the wgpu renderer instead.
    let use_wgpu = std::env::args().any(|a| a == "--wgpu");
    if use_wgpu {
        let mut game = create_game(DEFAULT_WIDTH, DEFAULT_HEIGHT, None);
        game.start_game();
        let mut runtime = hewn::wgpu::runtime::WindowRuntime::new();
        let _ = runtime.start(&mut game, hewn::wgpu::render::CameraStrategy::AllEntities);
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        play_snake_in_terminal();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_snake_in_terminal() {
    let mut game = create_game(DEFAULT_WIDTH, DEFAULT_HEIGHT, None);
    game.start_game();
    let mut runtime = TerminalRuntime::new_with_cursor_strategy(
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        Box::new(StaticCursorStrategy::new()),
    );
    runtime.start(&mut game);
}
