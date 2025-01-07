use crate::{display::Display, game::Game, FRAME_RATE_MILLIS, GAME_STEP_MILLIS};
use std::{
    thread,
    time::{self, Duration, Instant},
};

use super::game::BaseGame;

pub struct Control<'a> {
    pub stdin: termion::input::Keys<termion::AsyncReader>,
    pub game: &'a mut Game,
    pub display: &'a mut Display,
    last_frame_time: Instant,
}

impl Control<'_> {
    pub fn new<'a>(
        stdin: termion::input::Keys<termion::AsyncReader>,
        game: &'a mut Game,
        display: &'a mut Display,
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
                        self.game.set_player_control_key(Some(key));
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
            self.display.next(self.game);
        }
    }
}
