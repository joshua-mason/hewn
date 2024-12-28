use game::Direction;
use game::Game;
use termion;

fn main() {
    let (stdout, stdin) = io::initialize_terminal();
    let mut game = game::Game { player_position: 0 };
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
        pub player_position: usize,
    }

    pub enum Direction {
        Left,
        Right,
    }

    impl Game {
        pub fn player_move(&mut self, direction: Direction) {
            match direction {
                Direction::Left if self.player_position != 0 => {
                    self.player_position = self.player_position - 1;
                }
                Direction::Right if self.player_position < 5 => {
                    self.player_position = 1 + self.player_position;
                }
                _ => {}
            }
        }
    }
}

mod display {
    use std::io::Stdout;
    use std::io::Write;
    use termion::raw::RawTerminal;

    use crate::game::Game;

    pub struct Display {
        stdout: RawTerminal<Stdout>,
    }

    impl Display {
        pub fn next(&mut self, game: &Game) {
            write!(
                self.stdout,
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                Display::stringify_game(game),
                // key
            )
            .unwrap();

            self.stdout.lock().flush().unwrap();
        }

        pub fn new(stdout: RawTerminal<Stdout>) -> Display {
            Display { stdout }
        }

        fn stringify_game(game: &Game) -> String {
            let mut s = "......".to_owned();

            s.replace_range(game.player_position..(game.player_position + 1), "#");
            s
        }
    }
}

mod control {
    use crate::game::Game;
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
                thread::sleep(time::Duration::from_millis(50));
                on_render(self.game);
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
