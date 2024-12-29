use crate::{display::Display, game::Game, FRAME_RATE_MILLIS, GAME_STEP_MILLIS};
use std::{
    thread,
    time::{self, Duration, Instant},
};

pub enum PlayerControl {
    MovingLeft,
    MovingRight,
    Still,
}

pub struct Control<'a> {
    pub stdin: termion::input::Keys<termion::AsyncReader>,
    pub game: &'a mut Game,
    pub display: &'a mut Display,
    player_movement: PlayerControl,
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
            player_movement: PlayerControl::Still,
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
                    termion::event::Key::Left => {
                        self.player_movement = PlayerControl::MovingLeft;
                    }
                    termion::event::Key::Right => {
                        self.player_movement = PlayerControl::MovingRight;
                    }

                    _ => {
                        self.player_movement = PlayerControl::Still;
                    }
                }
            }
            thread::sleep(time::Duration::from_millis(FRAME_RATE_MILLIS));

            let now = time::Instant::now();
            if now - self.last_frame_time > Duration::from_millis(GAME_STEP_MILLIS) {
                self.game.next(&self.player_movement);
                self.last_frame_time = now;

                if input.is_none() {
                    self.player_movement = PlayerControl::Still;
                }
            }
            self.display.next(self.game);
        }
    }
}
