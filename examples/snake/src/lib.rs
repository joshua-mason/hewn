pub mod game;

use crate::game::create_game;
use hewn::runtime::GameHandler;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_in_canvas(width: u16, height: u16, seed: Option<u64>) {
    let mut game = create_game(width, height, seed);
    game.start_game();
    let mut runtime = hewn::wgpu::runtime::WindowRuntime::new();
    let _ = runtime.start(
        &mut game,
        hewn::wgpu::render::CameraStrategy::AllEntities,
        None,
    );
}
