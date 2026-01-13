use asciijump::game::{SCREEN_HEIGHT, SCREEN_WIDTH};
#[cfg(not(target_arch = "wasm32"))]
use hewn::terminal::runtime::TerminalRuntime;
use hewn::wgpu;

use crate::game::create_game;

pub mod game;

fn main() {
    // Default to terminal to keep `cargo run -p asciijump` predictable.
    // Use `--wgpu` to run the wgpu renderer instead.
    let use_wgpu = std::env::args().any(|a| a == "--wgpu");
    if use_wgpu {
        play_asciijump_in_wgpu();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        play_asciijump_in_terminal();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_asciijump_in_terminal() {
    let mut game = create_game(None);
    let mut runtime = TerminalRuntime::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    runtime.start(&mut game);
}

pub fn play_asciijump_in_wgpu() {
    let mut game = create_game(None);
    let mut runtime = wgpu::runtime::WindowRuntime::new();
    let player_entity_id = game.player_id;
    let _ = runtime.start(
        &mut game,
        wgpu::render::CameraStrategy::CameraFollow(player_entity_id),
    );
}
