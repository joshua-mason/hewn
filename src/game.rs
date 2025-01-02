use crate::{
    control::PlayerControl,
    game_object::{
        platform::Platform, player_character::PlayerCharacter, utils::detect_collision, Coordinate,
    },
};

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

    pub platforms: Vec<Platform>,
    pub player: PlayerCharacter,

    pub state: GameState,
    pub score: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
            platforms: vec![],
            player: PlayerCharacter::new(),
            state: GameState::Menu,
            score: 0,
        }
    }

    pub fn set_platforms(&mut self, platforms: Vec<Platform>) {
        self.platforms = platforms;
    }

    pub fn next(&mut self, player_control: &PlayerControl) {
        if self.state != GameState::InGame {
            return;
        }

        match player_control {
            PlayerControl::MovingLeft if self.player.coordinate.x > 0 => {
                self.player.coordinate.x -= 1;
            }
            PlayerControl::MovingRight if self.player.coordinate.x < self.width - 1 => {
                self.player.coordinate.x += 1;
            }
            _ => {}
        }

        // FIXME: can we improve the efficiency here? whole loop is not very good
        let collision_platform = self
            .platforms
            .iter()
            .find(|platform| detect_collision(*platform, &self.player));

        if let Some(platform) = collision_platform {
            if self.player.velocity < 0 {
                self.player.velocity = 5;
                self.player.coordinate.y = platform.coordinate.y;
                return;
            }
        }

        self.player.coordinate.y =
            (self.player.coordinate.y as isize + self.player.velocity).max(0) as usize;
        self.player.velocity -= 1;

        if self.player.velocity < -6 {
            self.end_game();
        }

        self.score = self.score.max(self.player.coordinate.y);
    }

    pub fn start_game(&mut self) {
        self.score = 0;
        self.player = PlayerCharacter::new();
        self.state = GameState::InGame;
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Lost(self.score);
    }
}

#[cfg(test)]
mod test {
    use super::Game;
    use crate::game_object::{platform::Platform, player_character::PlayerCharacter, Coordinate};

    #[test]
    fn test_jump() {
        let mut game = Game::new(10, 10);
        game.start_game();
        fast_forward(&mut game, 1);
        assert_eq!(game.player.coordinate.x, 1);
        assert_eq!(game.player.coordinate.y, 6);

        fast_forward(&mut game, 6);
        assert_eq!(game.player.coordinate.y, 15);

        fast_forward(&mut game, 4);
        assert_eq!(game.player.coordinate.y, 1);
    }

    #[test]
    fn test_start_on_platform() {
        let mut game = Game::new(10, 10);
        game.player = PlayerCharacter::from_tuple((2, 2, -5));
        game.set_platforms(Platform::from_tuples(&[(1, 2)]));
        game.start_game();

        fast_forward(&mut game, 1);
        assert_eq!(game.player.coordinate.x, 2);
        assert_eq!(game.player.coordinate.y, 2);
        assert_eq!(game.player.velocity, 5);

        fast_forward(&mut game, 1);
        assert_eq!(game.player.coordinate.y, 7);
        assert_eq!(game.player.velocity, 4);
    }

    #[test]
    fn test_hit_platform() {
        let mut game = Game::new(10, 10);
        game.set_platforms(Platform::from_tuples(&[(1, 8)]));
        game.start_game();
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player.coordinate.y, 6);
        fast_forward(&mut game, 9);
        assert_eq!(game.player.velocity, 5);
        assert_eq!(game.player.coordinate.y, 8);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
    }

    #[test]
    fn test_miss_platform() {
        let mut game = Game::new(10, 10);
        game.set_platforms(Platform::from_tuples(&[(3, 15)]));
        game.start_game();

        fast_forward(&mut game, 11);
        assert_eq!(game.player.coordinate.y, 1);
    }

    #[test]
    fn test_start_jump() {
        let mut game = Game::new(10, 20);
        let platforms = Platform::from_tuples(&[(1, 1)]);
        game.set_platforms(platforms);
        game.start_game();

        assert_eq!(game.player.coordinate.y, 1);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.x, 1);
        assert_eq!(game.player.coordinate.y, 6);

        fast_forward(&mut game, 10);
        assert_eq!(game.player.coordinate.y, 1);
        assert_eq!(game.player.velocity, 5);

        fast_forward(&mut game, 5);
        assert_eq!(game.player.coordinate.y, 16);
    }

    #[test]
    fn test_fell_to_bottom_under_platform() {
        let mut game = Game::new(10, 20);
        let platforms = Platform::from_tuples(&[(1, 3)]);
        game.set_platforms(platforms);
        game.player.velocity = 0;
        game.start_game();

        assert_eq!(game.player.coordinate.y, 1);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.x, 1);
        assert_eq!(game.player.coordinate.y, 1);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.x, 1);
        assert_eq!(game.player.coordinate.y, 0);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.x, 1);
        assert_eq!(game.player.coordinate.y, 0);
        fast_forward(&mut game, 10);
        assert_eq!(game.player.coordinate.y, 0);
    }

    fn fast_forward(game: &mut Game, n: u16) {
        for _ in 0..n {
            game.next(&crate::control::PlayerControl::Still);
        }
    }
}
