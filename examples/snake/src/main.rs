mod game;
use crate::game::create_game;
#[cfg(not(target_arch = "wasm32"))]
use hewn::runtime::GameHandler;
use hewn::wgpu::render::CameraStrategy;
use hewn::wgpu::render::Tilemap;

const SCREEN_WIDTH: u16 = 30;
const SCREEN_HEIGHT: u16 = 20;
const TILEMAP_BYTES: &[u8] = include_bytes!("./assets/monochrome_tilemap_transparent_packed.png");

fn main() {
    let mut game = create_game(SCREEN_WIDTH, SCREEN_HEIGHT, None);
    game.start_game();

    // Pick runtime:
    // - default: wgpu window (desktop)
    // - `--terminal`: ASCII terminal renderer
    let use_terminal = std::env::args().any(|a| a == "--terminal");

    if use_terminal {
        // Static camera so the full 30x30 map is visible (no viewport-follow).
        let mut runtime =
            hewn::terminal::runtime::TerminalRuntime::new_static(SCREEN_WIDTH, SCREEN_HEIGHT);
        runtime.start(&mut game);
    } else {
        let tilemap = Tilemap::new(TILEMAP_BYTES, 20, 20);
        let mut runtime = hewn::wgpu::runtime::WindowRuntime::new();
        let _ = runtime.start(&mut game, CameraStrategy::AllEntities, Some(tilemap));
    }
}
