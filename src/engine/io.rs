use super::{display::BaseDisplay, game::BaseGame};
#[cfg(not(target_arch = "wasm32"))]
use crate::engine::game::map_termion_key;
use crate::engine::game::Key;
use std::io::{self, Stdout};
use std::{
    thread,
    time::{self, Duration, Instant},
};
#[cfg(not(target_arch = "wasm32"))]
use termion::input::TermRead;
#[cfg(not(target_arch = "wasm32"))]
use termion::raw::{IntoRawMode, RawTerminal};

const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;

#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_terminal() -> (
    RawTerminal<Stdout>,
    termion::input::Keys<termion::AsyncReader>,
) {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = termion::async_stdin().keys();
    (stdout, stdin)
}

#[cfg(not(target_arch = "wasm32"))]
pub struct TerminalControl<'a> {
    pub stdin: termion::input::Keys<termion::AsyncReader>,
    pub game: &'a mut dyn BaseGame,
    pub display: &'a mut BaseDisplay,
    last_frame_time: Instant,
}

#[cfg(not(target_arch = "wasm32"))]
impl TerminalControl<'_> {
    pub fn new<'a>(
        stdin: termion::input::Keys<termion::AsyncReader>,
        game: &'a mut dyn BaseGame,
        display: &'a mut BaseDisplay,
    ) -> TerminalControl<'a> {
        TerminalControl {
            stdin,
            game,
            last_frame_time: Instant::now(),
            display,
        }
    }

    pub fn listen(&mut self) {
        loop {
            let input = self.stdin.next();

            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Char('q') => break,
                    key if key != termion::event::Key::Char(' ') => {
                        self.game.set_player_control_key(Some(map_termion_key(key)));
                    }
                    termion::event::Key::Char(' ') => {
                        self.game.start_game();
                    }
                    _ => {
                        self.game.set_player_control_key(None);
                    }
                }
            }

            thread::sleep(time::Duration::from_millis(FRAME_RATE_MILLIS));

            let now = time::Instant::now();
            if now - self.last_frame_time > Duration::from_millis(GAME_STEP_MILLIS) {
                self.game.next();
                self.last_frame_time = now;

                if input.is_none() {
                    self.game.set_player_control_key(None);
                }
            }
            self.display
                .next(&self.game.entities().game_objects, self.game.debug_str());
        }
    }
}

pub struct WebControl {
    game: Box<dyn BaseGame>,
    display: BaseDisplay,
}

impl WebControl {
    pub fn new(game: Box<dyn BaseGame>, display: BaseDisplay) -> WebControl {
        WebControl { game, display }
    }

    pub fn start(&mut self) {
        self.game.start_game();
    }

    pub fn set_player_control_key(&mut self, key: Option<Key>) {
        self.game.set_player_control_key(key);
    }

    pub fn tick(&mut self) {
        self.game.next();
    }

    pub fn render(&mut self) -> String {
        self.display
            .next(&self.game.entities().game_objects, self.game.debug_str())
    }
}
