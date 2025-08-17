//! # Hewn
//!
//! **Status:** Alpha – experimental game engine for learning.
//!
//! Includes examples: Snake, Doodle Jump, Flappy Bird.

use wasm_bindgen::prelude::*;

mod asciibird;
mod asciijump;
mod engine;
mod snake;

#[wasm_bindgen]
pub enum WasmKey {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}

#[wasm_bindgen]
pub struct Game {
    web_control: engine::control::WebControl,
}

#[wasm_bindgen]
impl Game {
    // generic but this specific constructure implements a specific game
    pub fn new_snake() -> Game {
        let width: u16 = 30;
        let height: u16 = 25;
        let web_control = engine::control::WebControl::new(
            Box::new(snake::game::Game::new(width as usize, height as usize)),
            engine::display::BaseDisplay {
                renderer: Box::new(engine::display::WebRenderer::new(height, width)),
                view_cursor: engine::game_object::Coordinate { x: 0, y: 0 },
                // this depends on the game
                cursor_strategy: Box::new(engine::display::cursor::StaticCursorStrategy::new()),
            },
        );
        Game { web_control }
    }

    pub fn new_asciijump() -> Game {
        let width: u16 = 30;
        let height: u16 = 25;
        let web_control = engine::control::WebControl::new(
            Box::new(asciijump::game::Game::new(width as usize, height as usize)),
            engine::display::BaseDisplay {
                renderer: Box::new(engine::display::WebRenderer::new(height, width)),
                view_cursor: engine::game_object::Coordinate { x: 0, y: 0 },
                cursor_strategy: Box::new(
                    engine::display::cursor::FollowPlayerYCursorStrategy::new(),
                ),
            },
        );
        Game { web_control }
    }

    pub fn new_asciibird() -> Game {
        let width: u16 = 30;
        let height: u16 = 25;
        let web_control = engine::control::WebControl::new(
            Box::new(asciibird::game::Game::new()),
            engine::display::BaseDisplay {
                renderer: Box::new(engine::display::WebRenderer::new(height, width)),
                view_cursor: engine::game_object::Coordinate { x: 0, y: 0 },
                cursor_strategy: Box::new(
                    engine::display::cursor::FollowPlayerXCursorStrategy::new(),
                ),
            },
        );
        Game { web_control }
    }

    // generic
    pub fn start(&mut self) {
        self.web_control.start();
    }

    // generic
    pub fn set_player_control_key(&mut self, key: Option<WasmKey>) {
        fn map_wasm_key(k: WasmKey) -> engine::game::Key {
            match k {
                WasmKey::Left => engine::game::Key::Left,
                WasmKey::Right => engine::game::Key::Right,
                WasmKey::Up => engine::game::Key::Up,
                WasmKey::Down => engine::game::Key::Down,
                WasmKey::Space => engine::game::Key::Space,
                WasmKey::Escape => engine::game::Key::Escape,
            }
        }
        self.web_control
            .set_player_control_key(key.map(map_wasm_key));
    }

    // generic
    pub fn tick(&mut self) {
        self.web_control.tick();
    }

    // generic
    pub fn render(&mut self) -> String {
        self.web_control.render()
    }
}
