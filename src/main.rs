use game::Direction;
use game::Game;
use termion;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const FRAME_RATE_MILLIS: u64 = 100;
const SCREEN_HEIGHT: u16 = 20;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game::new(WIDTH, HEIGHT);
    let mut control = control::Control {
        stdin,
        game: &mut game,
    };
    let mut display = display::Display::new(stdout);

    let mut on_user_input = |key: termion::event::Key, game: &mut Game| match key {
        termion::event::Key::Left => {
            game.player_move(Direction::Left);
        }
        termion::event::Key::Right => {
            game.player_move(Direction::Right);
        }
        _ => todo!(),
    };

    let mut on_game_render = |game: &Game| {
        display.next(&game);
    };

    control.listen(&mut on_user_input, &mut on_game_render);
}

mod game {
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

        pub fn player_move(&mut self, direction: Direction) {
            match direction {
                Direction::Left if self.player_pos_x != 0 => {
                    self.player_pos_x = self.player_pos_x - 1;
                }
                Direction::Right if self.player_pos_x < self.width - 1 => {
                    self.player_pos_x = 1 + self.player_pos_x;
                }
                _ => {}
            }
        }

        pub fn next(&mut self) {
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
    use crate::{game::Game, FRAME_RATE_MILLIS};
    use std::{thread, time};

    pub struct Control<'a> {
        pub stdin: termion::input::Keys<termion::AsyncReader>,
        pub game: &'a mut Game,
    }

    impl Control<'_> {
        pub fn listen<F, G>(&mut self, on_display: &mut F, on_render: &mut G)
        where
            F: FnMut(termion::event::Key, &mut Game),
            G: FnMut(&Game),
        {
            loop {
                let input = self.stdin.next();

                if let Some(Ok(key)) = input {
                    match key {
                        termion::event::Key::Char('q') => break,
                        _ => {
                            on_display(key, self.game);
                        }
                    }
                }
                thread::sleep(time::Duration::from_millis(FRAME_RATE_MILLIS));
                on_render(self.game);
                self.game.next();
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
