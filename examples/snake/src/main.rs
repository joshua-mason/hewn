mod game;
use crate::game::create_game;
#[cfg(not(target_arch = "wasm32"))]
use hewn::runtime::GameHandler;

const SCREEN_WIDTH: u16 = 50;
const SCREEN_HEIGHT: u16 = 50;

fn main() {
    let mut game = create_game(SCREEN_WIDTH, SCREEN_HEIGHT, None);
    game.start_game();
    let mut runtime = hewn::wgpu::runtime::WindowRuntime::new();
    let _ = runtime.start(&mut game);

    let mut game = create_game(SCREEN_WIDTH, SCREEN_HEIGHT, None);
    let mut runtime = hewn::terminal::runtime::TerminalRuntime::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    runtime.start(&mut game);
}
