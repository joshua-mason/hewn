use super::game_objects::platform::Platform;
use super::game_objects::player_character::PlayerCharacter;
use hewn::{
    game::{BaseGame, Entities, Key},
    game_object::{
        utils::{collision_pass, try_get_concrete_type, try_get_mut_concrete_type},
        GameObject,
    },
};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 500;
pub const SCREEN_WIDTH: u16 = 10;
pub const SCREEN_HEIGHT: u16 = 20;

pub fn default() -> Game {
    let mut game = Game::new(WIDTH, HEIGHT);
    let platforms = Platform::generate_platforms(WIDTH, HEIGHT);
    game.set_player(PlayerCharacter::new());
    game.set_platforms(platforms);
    game
}

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

    pub state: GameState,
    pub score: usize,

    pub entities: Entities,
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
            // game_objects: vec![],
            player_control_key: None,
        };
        game.set_player(PlayerCharacter::new());
        game
    }

    fn move_player(&mut self) {
        let width = self.width;
        match self.player_control_key {
            Some(Key::Left) => {
                if let Some(player) = self.get_mut_player_object() {
                    if player.coordinate.x > 0 {
                        player.move_left()
                    }
                }
            }
            Some(Key::Right) => {
                if let Some(player) = self.get_mut_player_object() {
                    if player.coordinate.x < width - 1 {
                        player.move_right()
                    }
                }
            }
            _ => {}
        }
    }
    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }

    pub fn get_player_object(&self) -> Option<&PlayerCharacter> {
        take_player_object(&self.entities().game_objects)
    }

    pub fn get_mut_player_object(&mut self) -> Option<&mut PlayerCharacter> {
        self.entities
            .game_objects
            .iter_mut()
            .filter_map(|o| try_get_mut_concrete_type::<PlayerCharacter>(&mut **o))
            .next()
    }

    pub fn set_platforms(&mut self, platforms: Vec<Platform>) {
        let mut game_objects = platforms
            .into_iter()
            .map(|p| Box::new(p) as Box<dyn GameObject>)
            .collect::<Vec<Box<dyn GameObject>>>();
        self.entities.add_game_objects(&mut game_objects);
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
}

impl BaseGame for Game {
    fn set_player_control_key(&mut self, key: Option<Key>) {
        self.player_control_key = key
    }
    fn entities(&self) -> &Entities {
        &self.entities
    }

    fn start_game(&mut self) {
        self.score = 0;
        self.get_mut_player_object().unwrap().reset();
        self.state = GameState::InGame;
    }

    fn next(&mut self) {
        if self.state != GameState::InGame {
            return;
        }

        /*
        So here we have a weird issue - we have collision detection happening after we move.
        This means in a test where we start on a platform, we fall through the platform...
        I think the core issue here is we haven't fully thought it through what we want to happen, so
        there is some "undefined" behaviour persay for the game.

        A manual version of this is to have a flag on the player object that we check if we collide, then don't
        move if that happens.. A specific fix could be to set veolicity to 0 on collision and then on nexwt turn
        set it to 5 as we are on top of it.. ?

        IDK maybe some research andd thinking to figure this one out!
        */

        self.move_player();

        // This and the collision pass are generic to all games, so I wonder if we can somehow refactor
        // this out - although I don't know if order matters in this case, or how opinionated to be,
        // as I suppose you might want to not have this part of your logic? but then you set the game objects
        // to not be able to collide I guess.
        self.entities
            .game_objects
            .iter_mut()
            .for_each(|o| o.next_step());

        collision_pass(&mut self.entities.game_objects);

        // FIXME: can we improve the efficiency here? whole loop is not very good
        // FIXME: when two platforms, we don't definitely hit the closest one

        if self.get_player_object().unwrap().velocity < -6 {
            self.end_game();
        }
        if self.get_player_object().unwrap().coordinate.y > self.height {
            // TODO add win state
            self.end_game();
        }

        self.score = self
            .score
            .max(self.get_player_object().unwrap().coordinate.y);
    }

    fn debug_str(&self) -> Option<String> {
        if let Some(player) = self.get_player_object() {
            let a = format!(
                "v = {:4}, x = {:3}, y = {:3}",
                player.velocity, player.coordinate.x, player.coordinate.y
            );
            Some(a)
        } else {
            None
        }
    }
}

// TODO: move to engine and split into take game object and take game objects
pub fn take_game_objects<T: GameObject>(game_objects: &[Box<dyn GameObject>]) -> Vec<&T> {
    game_objects
        .iter()
        .filter_map(|o| try_get_concrete_type::<T>(&**o))
        .collect::<Vec<&T>>()
}

pub fn take_player_object(game_objects: &[Box<dyn GameObject>]) -> Option<&PlayerCharacter> {
    take_game_objects::<PlayerCharacter>(game_objects)
        .into_iter()
        .next()
}

#[cfg(test)]
mod test {
    use crate::game_objects::{platform::Platform, player_character::PlayerCharacter};
    use hewn::{game::BaseGame, game_object::GameObject};

    use super::Game;
    #[test]
    fn test_jump() {
        let mut game = Game::new(10, 10);
        game.start_game();
        fast_forward(&mut game, 1);
        assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 6);

        fast_forward(&mut game, 6);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 15);

        fast_forward(&mut game, 4);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);
    }

    #[test]
    fn test_start_on_platform() {
        let mut game = Game::new(10, 10);
        let platforms = Platform::from_tuples(&[(2, 2)]);
        game.set_platforms(platforms);
        let player = PlayerCharacter::from_tuple((2, 3, -5));
        println!("on creation: {:?}", player);
        println!("added to game: {:?}", game.get_player_object());
        game.start_game();
        println!("Before player replacement: {:?}", game);
        game.set_player(player);
        println!("After player replacement: {:?}", game);
        println!("set player: {:?}", game.get_player_object());

        fast_forward(&mut game, 1);
        println!("{:?}", game);
        assert_eq!(game.get_player_object().unwrap().coordinate.x, 2);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 2);
        assert_eq!(game.get_player_object().unwrap().velocity, 5);

        fast_forward(&mut game, 1);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 7);
        assert_eq!(game.get_player_object().unwrap().velocity, 4);
    }

    #[test]
    fn test_hit_platform() {
        let mut game = Game::new(10, 10);
        game.set_platforms(Platform::from_tuples(&[(1, 8)]));

        game.start_game();

        game.next();

        assert_eq!(game.get_player_object().unwrap().coordinate.y, 6);
        fast_forward(&mut game, 9);
        assert_eq!(game.get_player_object().unwrap().velocity, 5);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 8);

        game.next();
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 13);
    }

    #[test]
    fn test_player_game_object_hit_platform() {
        let mut game = Game::new(10, 10);
        game.set_platforms(Platform::from_tuples(&[(1, 8)]));
        game.start_game();
        game.next();

        {
            let player_object = game.get_player_object().unwrap();

            println!("asdas:{:?}", player_object);
            assert_eq!(player_object.get_coords().y, 6);
        }
        fast_forward(&mut game, 1);
        {
            let player_object = game.get_player_object().unwrap();

            println!("asdas:{:?}", player_object);
            assert_eq!(player_object.get_coords().y, 10);
        }
        fast_forward(&mut game, 7);
        {
            let player_object = game.get_player_object().unwrap();

            println!("asdas:{:?}", player_object);
            assert_eq!(player_object.get_coords().y, 8);
        }
        println!("244");
        fast_forward(&mut game, 1);

        {
            let player_object = game.get_player_object().unwrap();

            println!("asdas:{:?}", player_object);
            assert_eq!(player_object.get_coords().y, 13);
        }
        // assert_eq!(game.get_player_object().unwrap().coordinate.y, 13);
    }

    #[test]
    fn test_miss_platform() {
        let mut game = Game::new(10, 10);
        game.set_platforms(Platform::from_tuples(&[(3, 15)]));
        game.start_game();

        fast_forward(&mut game, 11);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);
    }

    #[test]
    fn test_start_jump() {
        let mut game = Game::new(10, 20);
        let platforms = Platform::from_tuples(&[(1, 1)]);
        game.set_platforms(platforms);
        game.start_game();

        assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);

        game.next();
        assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 6);

        fast_forward(&mut game, 10);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);
        assert_eq!(game.get_player_object().unwrap().velocity, 5);

        fast_forward(&mut game, 5);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 16);
    }

    #[test]
    fn test_fell_to_bottom_under_platform() {
        let mut game = Game::new(10, 20);
        let platforms = Platform::from_tuples(&[(1, 3)]);
        game.set_platforms(platforms);
        game.start_game();
        game.get_mut_player_object().unwrap().velocity = 0;

        assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);

        game.next();
        assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 1);

        game.next();
        assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 0);

        game.next();
        assert_eq!(game.get_player_object().unwrap().coordinate.x, 1);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 0);
        fast_forward(&mut game, 10);
        assert_eq!(game.get_player_object().unwrap().coordinate.y, 0);
    }

    fn fast_forward(game: &mut Game, n: u16) {
        for _ in 0..n {
            game.next();
        }
    }
}
