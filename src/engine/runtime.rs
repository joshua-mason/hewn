//! Wasm and terminal game runtimes.

use crate::ecs::ComponentType;

use super::{game::GameLogic, view::View};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{self, Stdout};
use std::{
    thread,
    time::{self, Duration, Instant},
};
#[cfg(not(target_arch = "wasm32"))]
use termion::input::TermRead;
#[cfg(not(target_arch = "wasm32"))]
use termion::raw::{IntoRawMode, RawTerminal};
use wasm_bindgen::prelude::*;

const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;

/// Key for player control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}

/// Initialize terminal IO.
#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_terminal_io() -> (
    RawTerminal<Stdout>,
    termion::input::Keys<termion::AsyncReader>,
) {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = termion::async_stdin().keys();
    (stdout, stdin)
}

/// A runtime for a terminal game.
#[cfg(not(target_arch = "wasm32"))]
pub struct TerminalRuntime<'a> {
    pub stdin: termion::input::Keys<termion::AsyncReader>,
    pub game: &'a mut dyn GameLogic,
    pub display: &'a mut View,
    last_frame_time: Instant,
    player_control_key: Option<Key>,
}

#[cfg(not(target_arch = "wasm32"))]
impl TerminalRuntime<'_> {
    pub fn new<'a>(
        stdin: termion::input::Keys<termion::AsyncReader>,
        game: &'a mut dyn GameLogic,
        display: &'a mut View,
    ) -> TerminalRuntime<'a> {
        TerminalRuntime {
            stdin,
            game,
            last_frame_time: Instant::now(),
            display,
            player_control_key: None,
        }
    }

    /// Start the game loop listening for player input and rendering the game.
    pub fn start(&mut self) {
        loop {
            use crate::ecs::ComponentType;

            let input = self.stdin.next();

            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Char('q') => break,
                    key if key != termion::event::Key::Char(' ') => {
                        self.player_control_key = map_termion_key(key);
                    }
                    termion::event::Key::Char(' ') => {
                        self.game.start_game();
                    }
                    _ => {
                        self.player_control_key = None;
                    }
                }
            }
            thread::sleep(time::Duration::from_millis(FRAME_RATE_MILLIS));

            let now = time::Instant::now();
            if now - self.last_frame_time > Duration::from_millis(GAME_STEP_MILLIS) {
                self.game.next(self.player_control_key);
                self.last_frame_time = now;

                if input.is_none() {
                    self.player_control_key = None;
                }
            }
            let ecs = self.game.ecs();
            let entities = ecs.get_entities_by_component(ComponentType::render);
            self.display.next(entities, self.game.debug_str());
        }
    }
}

/// Map a termion key to a Hewn key.
#[cfg(not(target_arch = "wasm32"))]
pub fn map_termion_key(key: termion::event::Key) -> Option<Key> {
    match key {
        termion::event::Key::Left => Some(Key::Left),
        termion::event::Key::Right => Some(Key::Right),
        termion::event::Key::Up => Some(Key::Up),
        termion::event::Key::Down => Some(Key::Down),
        termion::event::Key::Char(' ') => Some(Key::Space),
        termion::event::Key::Esc => Some(Key::Escape),
        _ => None,
    }
}

/// A runtime for a web game.
pub struct WebRuntime {
    game: Box<dyn GameLogic>,
    display: View,
}

impl WebRuntime {
    /// Create a new web runtime.
    pub fn new(game: Box<dyn GameLogic>, display: View) -> WebRuntime {
        WebRuntime { game, display }
    }

    /// Start the game loop.
    pub fn start(&mut self) {
        self.game.start_game();
    }

    /// Compute the next game state based on player input.
    pub fn tick(&mut self, key: Option<WasmKey>) {
        self.game.next(map_wasm_key(key));
    }

    /// Render the game to a string.
    pub fn render(&mut self) -> String {
        let ecs = self.game.ecs();
        let entities = ecs.get_entities_by_component(ComponentType::render);
        self.display.next(entities, self.game.debug_str())
    }
}

/// A web game API.
#[wasm_bindgen]
pub struct WasmGameApi {
    web_runtime: WebRuntime,
}

#[wasm_bindgen]
impl WasmGameApi {
    pub fn start(&mut self) {
        self.web_runtime.start();
    }

    pub fn tick(&mut self, key: Option<WasmKey>) {
        self.web_runtime.tick(key);
    }

    pub fn render(&mut self) -> String {
        self.web_runtime.render()
    }
}

pub fn new_wasm_game_api(web_runtime: WebRuntime) -> WasmGameApi {
    WasmGameApi { web_runtime }
}

/// Map a web key to a Hewn key.
/// TODO: do we need this, or should we just expose the Hewn key enum?
fn map_wasm_key(k: Option<WasmKey>) -> Option<Key> {
    if k.is_none() {
        return None;
    }
    let k = k.unwrap();
    match k {
        WasmKey::Left => Some(Key::Left),
        WasmKey::Right => Some(Key::Right),
        WasmKey::Up => Some(Key::Up),
        WasmKey::Down => Some(Key::Down),
        WasmKey::Space => Some(Key::Space),
        WasmKey::Escape => Some(Key::Escape),
    }
}

#[wasm_bindgen]
pub enum WasmKey {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}
