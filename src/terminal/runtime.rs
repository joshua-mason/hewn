use crate::ecs::ComponentType;
use crate::runtime::GameHandler;
use crate::runtime::Key;
use crate::terminal::render::View;
use crate::terminal::render::{
    ScreenDimensions, TerminalRenderer, ViewCoordinate, cursor::FollowPlayerXYCursorStrategy,
};
use std::io::Stdout;
use std::thread;
use std::time::{self, Duration, Instant};
use termion::raw::RawTerminal;
pub(crate) const FRAME_RATE_MILLIS: u64 = 100;
pub(crate) const GAME_STEP_MILLIS: u64 = 100;

impl From<termion::event::Key> for Key {
    fn from(key: termion::event::Key) -> Self {
        match key {
            termion::event::Key::Left => Key::Left,
            termion::event::Key::Right => Key::Right,
            termion::event::Key::Up => Key::Up,
            termion::event::Key::Down => Key::Down,
            termion::event::Key::Char(' ') => Key::Space,
            termion::event::Key::Esc => Key::Escape,
            _ => panic!("Unsupported key: {:?}", key),
        }
    }
}

/// Initialize terminal IO.
pub fn initialize_terminal_io() -> (
    RawTerminal<Stdout>,
    termion::input::Keys<termion::AsyncReader>,
) {
    use std::io;

    use termion::{input::TermRead, raw::IntoRawMode};

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = termion::async_stdin().keys();
    (stdout, stdin)
}

/// A runtime for a terminal game.
pub struct TerminalRuntime {
    pub stdin: termion::input::Keys<termion::AsyncReader>,
    pub display: View,
    pub(crate) last_frame_time: Instant,
    pub(crate) player_control_key: Option<Key>,
}

impl TerminalRuntime {
    pub fn new(width: u16, height: u16) -> TerminalRuntime {
        let (stdout, stdin) = initialize_terminal_io();

        let view = View {
            view_cursor: ViewCoordinate { x: 0, y: 0 },
            renderer: Box::new(TerminalRenderer::new(
                stdout,
                ScreenDimensions {
                    x: width,
                    y: height,
                },
            )),
            cursor_strategy: Box::new(FollowPlayerXYCursorStrategy::new()),
        };

        TerminalRuntime {
            stdin,
            last_frame_time: Instant::now(),
            display: view,
            player_control_key: None,
        }
    }

    /// Start the game loop listening for player input and rendering the game.
    pub fn start(&mut self, game: &mut dyn GameHandler) {
        loop {
            let input = self.stdin.next();

            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Char('q') => break,
                    key if key != termion::event::Key::Char(' ') => {
                        game.handle_key(key.into(), true);
                    }
                    termion::event::Key::Char(' ') => {
                        game.start_game();
                    }
                    _ => {
                        self.player_control_key = None;
                    }
                }
            } else {
                game.handle_key(Key::Up, false);
                game.handle_key(Key::Down, false);
                game.handle_key(Key::Left, false);
                game.handle_key(Key::Right, false);
            }
            thread::sleep(time::Duration::from_millis(FRAME_RATE_MILLIS));

            let now = time::Instant::now();
            if now - self.last_frame_time > Duration::from_millis(GAME_STEP_MILLIS) {
                game.next();
                self.last_frame_time = now;

                if input.is_none() {
                    self.player_control_key = None;
                }
            }
            let ecs = game.ecs();
            let entities = ecs.get_entities_by(ComponentType::Render);
            self.display.next(entities, game.debug_str());
        }
    }
}
