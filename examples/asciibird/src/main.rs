mod game;

use crate::game::create_game;
#[cfg(not(target_arch = "wasm32"))]
use hewn::terminal::runtime::TerminalRuntime;
use hewn::wgpu;

pub const SCREEN_WIDTH: u16 = 50;
pub const SCREEN_HEIGHT: u16 = 30;

fn main() {
    // Default to terminal to keep `cargo run -p asciibird` predictable.
    // Use `--wgpu` to run the wgpu renderer instead.
    let use_wgpu = std::env::args().any(|a| a == "--wgpu");
    if use_wgpu {
        play_asciibird_in_wgpu();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        play_asciibird_in_terminal();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn play_asciibird_in_terminal() {
    let mut game = create_game(None);
    let mut runtime = TerminalRuntime::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    runtime.start(&mut game);
}

fn play_asciibird_in_wgpu() {
    let mut game = create_game(None);
    let player_entity_id = game.player_id;
    let mut runtime = wgpu::runtime::WindowRuntime::new();
    let _ = runtime.start(
        &mut game,
        wgpu::render::CameraStrategy::CameraFollow(player_entity_id),
    );
}
