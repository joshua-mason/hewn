// TODO
// * Render platforms
// * Jump on hitting a platform
// * Move screen with the player
// * Points tally

use game::Game;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;
const SCREEN_HEIGHT: u16 = 20;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let mut display = display::Display::new(stdout);
    let mut control = control::Control::new(stdin, &mut game, &mut display);

    control.listen();
}

mod game {
    use crate::control::PlayerMovement;

    pub struct Game {
        pub player_pos_x: usize,
        pub player_pos_y: isize,
        pub player_velocity: isize,
        pub width: usize,
        pub height: usize,
    }

    pub enum Direction {
        Left,
        Right,
    }

    struct Coordinate {
        x: u16,
        y: u16,
    }

    struct Platform {
        coordinate: Coordinate,
        length: u8,
    }

    impl Game {
        pub fn new(width: usize, height: usize) -> Game {
            Game {
                player_pos_x: 1,
                player_pos_y: 1,
                player_velocity: 5,
                width,
                height,
            }
        }

        pub fn next(&mut self, player_movement: &PlayerMovement) {
            match player_movement {
                PlayerMovement::MovingLeft if self.player_pos_x > 0 => {
                    self.player_pos_x -= 1;
                }
                PlayerMovement::MovingRight if self.player_pos_x < self.width - 1 => {
                    self.player_pos_x += 1;
                }
                _ => {}
            }
            self.player_velocity -= 1;
            self.player_pos_y += self.player_velocity;
            if self.player_pos_y <= 1 {
                self.player_velocity = 5;
            }
        }
    }
}

mod display {
    use std::io::Stdout;
    use std::io::Write;
    use termion::raw::RawTerminal;

    use crate::game::Game;
    use crate::SCREEN_HEIGHT;
    use crate::WIDTH;

    pub struct Display {
        stdout: RawTerminal<Stdout>,
    }

    impl Display {
        pub fn next(&mut self, game: &Game) {
            write!(
                self.stdout,
                "{}{}{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                Display::player_level(game),
                termion::cursor::Goto(1, SCREEN_HEIGHT + 2),
                Display::debug(game),
            )
            .unwrap();

            self.stdout.lock().flush().unwrap();
        }

        pub fn new(stdout: RawTerminal<Stdout>) -> Display {
            Display { stdout }
        }

        fn debug(game: &Game) -> String {
            format!(
                "v = {:4}, x = {:3}, y = {:3}",
                game.player_velocity, game.player_pos_x, game.player_pos_y
            )
        }

        fn player_level(game: &Game) -> String {
            let mut display_string = "".to_owned();
            for height in 0..(SCREEN_HEIGHT) {
                let mut level = Display::build_level_string();
                if ((SCREEN_HEIGHT - height) == game.player_pos_y as u16) {
                    level.replace_range(game.player_pos_x..(game.player_pos_x + 1), "#");
                }
                level.push_str(&termion::cursor::Goto(1, height).to_string());
                display_string.push_str(&level);
            }
            display_string
        }

        fn build_level_string() -> String {
            let whitespaces = std::iter::repeat('.').take(WIDTH);
            let whitespaces = Vec::from_iter(whitespaces);
            let level = String::from_iter(whitespaces);
            level
        }
    }
}

mod control {
    use crate::{display::Display, game::Game, FRAME_RATE_MILLIS, GAME_STEP_MILLIS};
    use std::{
        thread,
        time::{self, Duration, Instant},
    };

    pub enum PlayerMovement {
        MovingLeft,
        MovingRight,
        Still,
    }

    pub struct Control<'a> {
        pub stdin: termion::input::Keys<termion::AsyncReader>,
        pub game: &'a mut Game,
        pub display: &'a mut Display,
        player_movement: PlayerMovement,
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
                player_movement: PlayerMovement::Still,
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
                            self.player_movement = PlayerMovement::MovingLeft;
                        }
                        termion::event::Key::Right => {
                            self.player_movement = PlayerMovement::MovingRight;
                        }

                        _ => {
                            self.player_movement = PlayerMovement::Still;
                        }
                    }
                } else {
                }
                thread::sleep(time::Duration::from_millis(FRAME_RATE_MILLIS));

                let now = time::Instant::now();
                if now - self.last_frame_time > Duration::from_millis(GAME_STEP_MILLIS) {
                    self.game.next(&self.player_movement);
                    self.last_frame_time = now;

                    if input.is_none() {
                        self.player_movement = PlayerMovement::Still;
                    }
                }
                self.display.next(self.game);
            }
        }
    }
}

mod io {
    use std::io::{self, Stdout};
    use termion::input::TermRead;
    use termion::raw::{IntoRawMode, RawTerminal};

    pub fn initialize_terminal() -> (
        RawTerminal<Stdout>,
        termion::input::Keys<termion::AsyncReader>,
    ) {
        let stdout = io::stdout().into_raw_mode().unwrap();

        let stdin = termion::async_stdin().keys();
        (stdout, stdin)
    }
}
