use hewn::game_object::Coordinate;
use hewn::runtime::{initialize_terminal_io, TerminalRuntime};
use hewn::view::cursor::StaticCursorStrategy;
use hewn::view::{TerminalRenderer, View};

const SCREEN_HEIGHT: u16 = 20;
const SCREEN_WIDTH: u16 = 50;

mod game_objects {
    use std::any::Any;

    use hewn::game_object::{CollisionBox, Coordinate, GameObject};

    #[derive(Debug)]
    pub struct Player {
        pub coords: Coordinate,
    }

    impl Player {
        pub fn new(x: usize, y: usize) -> Player {
            Player {
                coords: Coordinate { x, y },
            }
        }

        const WIDTH: usize = 1;
    }

    impl GameObject for Player {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_mut_any(&mut self) -> &mut dyn Any {
            self
        }

        fn collide(&mut self, _other: &dyn GameObject) {}

        fn display(&self) -> String {
            "@".to_owned()
        }

        fn get_collision_box(&self) -> CollisionBox {
            CollisionBox {
                x: self.coords.x..(self.coords.x + self.width()),
                y: self.coords.y..(self.coords.y + 1),
            }
        }

        fn get_coords(&self) -> &Coordinate {
            &self.coords
        }

        fn next_step(&mut self) {}

        fn priority(&self) -> u8 {
            1
        }

        fn width(&self) -> usize {
            Self::WIDTH
        }

        fn is_player(&self) -> bool {
            true
        }
    }
}

mod game {
    use hewn::{
        game::{Entities, GameLogic},
        game_object::{utils, GameObject},
        runtime::Key,
    };

    use crate::game_objects::Player;

    pub struct MinimalGame {
        entities: Entities,
        started: bool,
    }

    impl MinimalGame {
        pub fn new() -> MinimalGame {
            let mut entities = Entities::new();
            let mut objects: Vec<Box<dyn GameObject>> = vec![Box::new(Player::new(5, 5))];
            entities.add_game_objects(&mut objects);
            MinimalGame {
                entities,
                started: false,
            }
        }

        fn move_player(&mut self, key: Key) {
            if let Some(p) = utils::downcast_muts::<Player>(&mut self.entities.game_objects)
                .into_iter()
                .next()
            {
                match key {
                    Key::Left => {
                        p.coords.x = p.coords.x.saturating_sub(1);
                    }
                    Key::Right => {
                        p.coords.x = p.coords.x.saturating_add(1);
                    }
                    Key::Up => {
                        p.coords.y = p.coords.y.saturating_add(1);
                    }
                    Key::Down => {
                        p.coords.y = p.coords.y.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }

    impl GameLogic for MinimalGame {
        fn start_game(&mut self) {
            self.started = true;
        }

        fn next(&mut self, key: Option<Key>) {
            if !self.started {
                return;
            }
            if let Some(k) = key {
                self.move_player(k);
            }
        }

        fn entities(&self) -> &Entities {
            &self.entities
        }

        fn debug_str(&self) -> Option<String> {
            let player = utils::take_game_object::<Player>(&self.entities.game_objects)?;
            let c = player.get_coords();
            let start_game_str = if self.started {
                "Started"
            } else {
                "Hit Space to Start"
            };
            Some(format!("Player @ ({}, {}) {}", c.x, c.y, start_game_str))
        }
    }
}

fn main() {
    let (stdout, stdin) = initialize_terminal_io();

    let mut view = View {
        view_cursor: Coordinate { x: 0, y: 0 },
        renderer: Box::new(TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH)),
        cursor_strategy: Box::new(StaticCursorStrategy::new()),
    };

    let mut game = game::MinimalGame::new();
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut view);
    runtime.start();
}
