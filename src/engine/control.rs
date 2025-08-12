use crate::engine::game::termion_key_to_key;

use super::{display::BaseDisplay, game::BaseGame};
use std::{
    thread,
    time::{self, Duration, Instant},
};

const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;

pub struct Control<'a> {
    pub stdin: termion::input::Keys<termion::AsyncReader>,
    pub game: &'a mut dyn BaseGame,
    pub display: &'a mut dyn BaseDisplay,
    last_frame_time: Instant,
}

impl Control<'_> {
    pub fn new<'a>(
        stdin: termion::input::Keys<termion::AsyncReader>,
        game: &'a mut dyn BaseGame,
        display: &'a mut dyn BaseDisplay,
    ) -> Control<'a> {
        Control {
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
                        self.game
                            .set_player_control_key(Some(termion_key_to_key(key)));
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
                .next(self.game.game_objects(), self.game.debug_str());
        }
    }
}
