pub mod game;
use crate::game::create_game;
#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;
use hewn::runtime::GameHandler;
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys;

#[wasm_bindgen]
pub fn run_in_canvas(width: u16, height: u16, seed: Option<u64>) {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }
    let mut game = create_game(width, height, seed);
    game.start_game();
    let mut runtime = hewn::wgpu::runtime::WindowRuntime::new();
    let _ = runtime.start(&mut game, hewn::wgpu::render::CameraStrategy::AllEntities);
}
