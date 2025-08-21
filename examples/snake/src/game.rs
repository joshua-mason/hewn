use crate::game::game_objects::{player_character::PlayerCharacter, wall::Wall};

pub const WIDTH: usize = 30;
pub const HEIGHT: usize = 25;

pub fn default() -> snake::Game {
    let mut game = snake::Game::new(WIDTH, HEIGHT);
    let walls = Wall::generate_walls(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_walls(walls);
    game.generate_food();
    game
}

pub mod game_objects {
    pub mod player_character {
        use hewn::display::build_string;
        use hewn::game_object::utils::try_get_concrete_type;
        use hewn::game_object::{
            GameObject, {CollisionBox, Coordinate},
        };
        use std::any::Any;

        use super::{food::Food, snake_body::SnakeBody, wall::Wall};

        #[derive(Debug, PartialEq, Clone)]
        pub enum Direction {
            Left,
            Up,
            Right,
            Down,
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct PlayerCharacter {
            pub coordinate: Coordinate,
            pub direction: Direction,
            pub size: u16,
            pub hit_wall: bool,
        }

        impl PlayerCharacter {
            pub fn new() -> PlayerCharacter {
                PlayerCharacter {
                    coordinate: Coordinate { x: 1, y: 1 },
                    direction: Direction::Up,
                    size: 1,
                    hit_wall: false,
                }
            }

            #[cfg(test)]
            pub fn from_tuple(tuple: (usize, usize)) -> PlayerCharacter {
                PlayerCharacter {
                    coordinate: Coordinate {
                        x: tuple.0,
                        y: tuple.1,
                    },
                    ..Default::default()
                }
            }

            pub fn turn(&mut self, direction: Direction) {
                if direction == self.direction {
                    return;
                }
                let is_u_turn = match direction {
                    Direction::Left => self.direction == Direction::Right,
                    Direction::Up => self.direction == Direction::Down,
                    Direction::Right => self.direction == Direction::Left,
                    Direction::Down => self.direction == Direction::Up,
                };
                if is_u_turn {
                    return;
                }
                self.direction = direction;
            }

            pub fn reset(&mut self) {
                self.coordinate.x = 1;
                self.coordinate.y = 1;
                self.direction = Direction::Up
            }
        }

        impl Default for PlayerCharacter {
            fn default() -> Self {
                Self::new()
            }
        }

        impl GameObject for PlayerCharacter {
            fn get_collision_box(&self) -> CollisionBox {
                let coords = self.get_coords();
                let next_coords = match self.direction {
                    Direction::Down => (self.coordinate.x, self.coordinate.y - 1),
                    Direction::Left => (self.coordinate.x - 1, self.coordinate.y),
                    Direction::Up => (self.coordinate.x, self.coordinate.y + 1),
                    Direction::Right => (self.coordinate.x + 1, self.coordinate.y),
                };

                CollisionBox {
                    x: coords.x..(next_coords.0 + 1),
                    y: coords.y..(next_coords.1 + 1),
                }
            }

            fn collide(&mut self, other: &dyn GameObject) {
                if let Some(_food) = try_get_concrete_type::<Food>(other) {
                    self.size += 1;
                }
                if let Some(_wall) = try_get_concrete_type::<Wall>(other) {
                    self.hit_wall = true;
                }
                if let Some(_wall) = try_get_concrete_type::<SnakeBody>(other) {
                    self.hit_wall = true;
                }
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_mut_any(&mut self) -> &mut dyn Any {
                self
            }
            fn display(&self) -> String {
                build_string('0', 1)
            }
            fn width(&self) -> usize {
                1
            }
            fn priority(&self) -> u8 {
                0
            }
            fn next_step(&mut self) {
                // TODO refactor to function. is this the same for the other objects? should it be required in the engine?
                let next_coords = match self.direction {
                    Direction::Down => (self.coordinate.x, self.coordinate.y - 1),
                    Direction::Left => (self.coordinate.x - 1, self.coordinate.y),
                    Direction::Up => (self.coordinate.x, self.coordinate.y + 1),
                    Direction::Right => (self.coordinate.x + 1, self.coordinate.y),
                };
                self.coordinate.x = next_coords.0;
                self.coordinate.y = next_coords.1;
            }

            fn get_coords(&self) -> &Coordinate {
                &self.coordinate
            }
            fn is_player(&self) -> bool {
                true
            }
        }
    }

    pub mod food {

        use hewn::{
            display::build_string,
            game_object::{CollisionBox, Coordinate, GameObject},
        };
        use std::any::Any;

        #[derive(Debug, PartialEq, Clone)]
        pub struct Food {
            pub coordinate: Coordinate,
            pub eaten: bool,
        }

        impl Food {
            pub fn new() -> Food {
                Food {
                    coordinate: Coordinate { x: 3, y: 3 },
                    eaten: false,
                }
            }

            #[cfg(test)]
            pub fn from_tuple(tuple: (usize, usize)) -> Food {
                Food {
                    coordinate: Coordinate {
                        x: tuple.0,
                        y: tuple.1,
                    },
                    ..Default::default()
                }
            }
        }

        impl Default for Food {
            fn default() -> Self {
                Self::new()
            }
        }

        impl GameObject for Food {
            // TODO This seems confusing to understand without looking at examples (e.g. what if it is just the one spot?)
            // is there a way to make this easier? or is there a way to provide default implementations?
            fn get_collision_box(&self) -> CollisionBox {
                let coords = self.get_coords();

                if self.eaten {
                    return CollisionBox {
                        x: coords.x..(coords.x),
                        y: coords.y..(coords.y),
                    };
                }

                CollisionBox {
                    x: coords.x..(coords.x + 1),
                    y: coords.y..(coords.y + 1),
                }
            }

            fn collide(&mut self, _other: &dyn GameObject) {
                // TODO: destroy? let's mark as eaten: true, and clean up in game
                // but it is another place where creation/deletion of game objects
                // has been considered inside a gameobject.
                // only if it is the snake head?
                self.eaten = true;
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_mut_any(&mut self) -> &mut dyn Any {
                self
            }
            fn display(&self) -> String {
                if self.eaten {
                    return build_string('.', 1);
                }
                build_string('+', 1)
            }
            fn width(&self) -> usize {
                1
            }
            fn priority(&self) -> u8 {
                0
            }
            fn next_step(&mut self) {}

            fn get_coords(&self) -> &Coordinate {
                &self.coordinate
            }
            fn is_player(&self) -> bool {
                false
            }
        }
    }

    pub mod snake_body {
        use std::any::Any;

        use hewn::{
            display::build_string,
            game_object::{CollisionBox, Coordinate, GameObject},
        };

        #[derive(Debug)]
        pub struct SnakeBody {
            coordinate: Coordinate,
        }

        impl SnakeBody {
            pub fn new(coord: Coordinate) -> SnakeBody {
                SnakeBody { coordinate: coord }
            }
        }

        impl GameObject for SnakeBody {
            // TODO This seems confusing to understand without looking at examples (e.g. what if it is just the one spot?)
            // is there a way to make this easier? or is there a way to provide default implementations?
            fn get_collision_box(&self) -> CollisionBox {
                let coords = self.get_coords();

                CollisionBox {
                    x: coords.x..(coords.x + 1),
                    y: coords.y..(coords.y + 1),
                }
            }

            fn collide(&mut self, _other: &dyn GameObject) {}

            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_mut_any(&mut self) -> &mut dyn Any {
                self
            }
            fn display(&self) -> String {
                build_string('o', 1)
            }
            fn width(&self) -> usize {
                1
            }
            fn priority(&self) -> u8 {
                0
            }
            fn next_step(&mut self) {}

            fn get_coords(&self) -> &Coordinate {
                &self.coordinate
            }
            fn is_player(&self) -> bool {
                false
            }
        }
    }

    pub mod wall {
        use hewn::{
            display::build_string,
            game_object::{CollisionBox, Coordinate, GameObject},
        };
        use std::any::Any;

        #[derive(Debug, PartialEq, Clone)]
        pub struct Wall {
            pub coordinate: Coordinate,
        }

        impl Wall {
            pub fn from_tuple(coords: (usize, usize)) -> Wall {
                Wall {
                    coordinate: Coordinate {
                        x: coords.0,
                        y: coords.1,
                    },
                }
            }

            #[cfg(test)]
            pub fn from_tuples(tuples: &[(usize, usize)]) -> Vec<Wall> {
                tuples
                    .iter()
                    .map(|tuple| Wall::from_tuple(*tuple))
                    .collect::<Vec<_>>()
            }

            pub fn generate_walls(width: usize, height: usize) -> Vec<Wall> {
                let mut walls: Vec<Wall> = vec![];
                for x_index in 0..width {
                    walls.push(Wall::from_tuple((x_index, 1)));
                    walls.push(Wall::from_tuple((x_index, height - 2)));
                }
                for y_index in 0..height {
                    walls.push(Wall::from_tuple((0, y_index)));
                    walls.push(Wall::from_tuple((width - 1, y_index)));
                }

                walls
            }
        }

        // TODO should we offer this as part of the engine? since it is such an obvious use case
        impl GameObject for Wall {
            fn get_collision_box(&self) -> CollisionBox {
                let coords = self.get_coords();

                CollisionBox {
                    x: coords.x..(coords.x + 1),
                    y: coords.y..(coords.y + 1),
                }
            }

            fn collide(&mut self, _: &dyn GameObject) {}
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_mut_any(&mut self) -> &mut dyn Any {
                self
            }
            fn display(&self) -> String {
                build_string('#', 3)
            }
            fn width(&self) -> usize {
                1
            }
            fn priority(&self) -> u8 {
                1
            }
            fn next_step(&mut self) {}
            fn get_coords(&self) -> &Coordinate {
                &self.coordinate
            }
            fn is_player(&self) -> bool {
                false
            }
        }
    }
}

pub mod snake {
    use super::game_objects::food::Food;
    use super::game_objects::player_character::{Direction, PlayerCharacter};
    use super::game_objects::wall::Wall;
    use hewn::game::{BaseGame, Entities, Key};

    use hewn::game_object::utils::{
        collision_pass, take_game_object, try_get_concrete_type, try_get_mut_concrete_type,
    };
    use hewn::game_object::{Coordinate, GameObject};
    use rand::Rng;

    #[derive(Debug, PartialEq, Eq)]
    pub enum GameState {
        InGame,
        Menu,
        Lost(usize),
    }

    #[derive(Debug)]
    pub struct Game {
        pub width: usize,
        pub height: usize,

        state: GameState,
        pub score: usize,

        entities: Entities,
        player_control_key: Option<Key>,
    }

    impl Game {
        pub fn new(width: usize, height: usize) -> Game {
            let mut game = Game {
                width,
                height,
                state: GameState::Menu,
                score: 0,
                entities: Entities::new(),
                player_control_key: None,
            };
            game.set_player(PlayerCharacter::new());
            game
        }

        fn move_player(&mut self) {
            // TODO this is very verbose... I wonder if there is something we can do to help simplify what's going on here
            // can't get_mut_player_object earlier because then we can't access self.player_control_key
            match self.player_control_key {
                Some(Key::Left) => {
                    if let Some(player) = self.get_mut_player_object() {
                        player.turn(Direction::Left);
                    }
                }
                Some(Key::Right) => {
                    if let Some(player) = self.get_mut_player_object() {
                        player.turn(Direction::Right);
                    }
                }
                Some(Key::Up) => {
                    if let Some(player) = self.get_mut_player_object() {
                        player.turn(Direction::Up);
                    }
                }
                Some(Key::Down) => {
                    if let Some(player) = self.get_mut_player_object() {
                        player.turn(Direction::Down);
                    }
                }
                _ => {}
            }
        }

        pub fn end_game(&mut self) {
            self.state = GameState::Lost(self.score);
        }

        pub fn get_player_object(&self) -> Option<&PlayerCharacter> {
            take_game_object::<PlayerCharacter>(&self.entities().game_objects)
        }

        // pub fn get_food_objects(&mut self) -> Vec<&mut Food> {
        //     take_mut_game_objects::<Food>(self.entities.game_objects.iter_mut().as_mut_slice())
        // }

        pub fn get_mut_player_object(&mut self) -> Option<&mut PlayerCharacter> {
            self.entities
                .game_objects
                .iter_mut()
                .filter_map(|o| try_get_mut_concrete_type::<PlayerCharacter>(&mut **o))
                .next()
        }

        pub fn set_player(&mut self, player: PlayerCharacter) {
            let mut game_objects: Vec<Box<dyn GameObject>> = vec![Box::new(player)];
            if let Some(index) = self
                .entities
                .game_objects
                .iter()
                .position(|o| try_get_concrete_type::<PlayerCharacter>(&**o).is_some())
            {
                self.entities.game_objects.remove(index);
            }
            self.entities.add_game_objects(&mut game_objects);
        }

        // TODO can we just make this use generics rather than repeating each time?
        pub fn set_food(&mut self, food: Food) {
            let mut game_objects: Vec<Box<dyn GameObject>> = vec![Box::new(food)];
            if let Some(index) = self
                .entities
                .game_objects
                .iter()
                .position(|o| try_get_concrete_type::<Food>(&**o).is_some())
            {
                self.entities.game_objects.remove(index);
            }
            self.entities.add_game_objects(&mut game_objects);
        }

        pub fn set_walls(&mut self, walls: Vec<Wall>) {
            let mut game_objects = walls
                .into_iter()
                .map(|p| Box::new(p) as Box<dyn GameObject>)
                .collect::<Vec<Box<dyn GameObject>>>();
            self.entities.add_game_objects(&mut game_objects);
        }

        pub fn generate_food(&mut self) {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            // TODO we need to avoid generating on the snakes body/head
            self.set_food(Food {
                coordinate: Coordinate { x, y },
                eaten: false,
            });
        }
    }

    impl BaseGame for Game {
        // duplication across games - consider options to refactor out?
        fn set_player_control_key(&mut self, key: Option<Key>) {
            self.player_control_key = key
        }

        // duplication across games - consider options to refactor out?
        fn start_game(&mut self) {
            self.score = 0;
            self.get_mut_player_object().unwrap().reset();
            self.state = GameState::InGame;
        }

        // duplication across games - consider options to refactor out?
        fn entities(&self) -> &Entities {
            &self.entities
        }

        fn next(&mut self) {
            if self.state != GameState::InGame {
                return;
            }

            self.move_player();
            self.entities
                .game_objects
                .iter_mut()
                .for_each(|o| o.next_step());

            collision_pass(&mut self.entities.game_objects);

            if self.get_player_object().unwrap().hit_wall {
                self.end_game();
            }

            // Keep eaten food in place for one tick so tests can observe state change

            self.score = self
                .score
                .max(self.get_player_object().unwrap().coordinate.x);
        }

        fn debug_str(&self) -> Option<String> {
            if let Some(player) = self.get_player_object() {
                let a = format!(
                    "s = {:4}, x = {:3}, y = {:3}, d = {:?}, hit_wall = {:?}",
                    player.size,
                    player.coordinate.x,
                    player.coordinate.y,
                    player.direction,
                    player.hit_wall
                );
                Some(a)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    impl Game {
        pub fn entities_for_test(&self) -> &Entities {
            &self.entities
        }
    }
}

#[cfg(test)]
mod test {
    use hewn::{
        game::BaseGame,
        game_object::utils::{detect_collision, take_game_object},
    };

    use super::game_objects::player_character::PlayerCharacter;
    use super::{game_objects::food::Food, *};

    #[test]
    fn test_eat_food() {
        // let (stdout, stdin) = initialize_terminal();
        let mut game = crate::game::snake::Game::new(WIDTH, HEIGHT);
        // let walls = Wall::generate_walls(WIDTH, HEIGHT);
        game.set_player(PlayerCharacter::new());
        // game.set_walls(walls);
        game.set_food(Food::from_tuple((1, 2)));

        let food = take_game_object::<Food>(&game.entities_for_test().game_objects);
        let player = take_game_object::<PlayerCharacter>(&game.entities_for_test().game_objects);
        assert!(detect_collision(food.unwrap(), player.unwrap()));
        println!("Next");
        game.start_game();
        game.next();
        println!("Done {:?}", game.entities_for_test().game_objects);

        let food = take_game_object::<Food>(&game.entities_for_test().game_objects);
        let player = take_game_object::<PlayerCharacter>(&game.entities_for_test().game_objects);

        assert!(food.unwrap().eaten);
        assert!(!detect_collision(food.unwrap(), player.unwrap()));
    }
}
