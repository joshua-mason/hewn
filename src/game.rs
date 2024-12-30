use crate::{
    control::PlayerControl,
    game_object::{detect_collision, Collide, CollisionBox, Coordinate, Platform, PlayerCharacter},
};

#[derive(Debug)]
pub struct Game {
    pub width: usize,
    pub height: usize,

    pub platforms: Vec<Platform>,
    pub player: PlayerCharacter,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
            platforms: vec![
                Platform {
                    coordinate: Coordinate { x: 5, y: 5 },
                    length: 3,
                },
                Platform {
                    coordinate: Coordinate { x: 5, y: 10 },
                    length: 3,
                },
            ],
            player: PlayerCharacter {
                coordinate: Coordinate { x: 1, y: 1 },
                velocity: 5,
            },
        }
    }

    pub fn next(&mut self, player_control: &PlayerControl) {
        // check platform collision:
        // FIXME: can we improve the efficiency here? whole loop is not very good
        let collision_platform = self
            .platforms
            .iter()
            .find(|platform| detect_collision(*platform, &self.player));
        self.player.coordinate.y =
            ((self.player.coordinate.y as isize + self.player.velocity).max(0) as usize);

        if self.player.coordinate.y <= 1 {
            self.player.velocity = 5;
        } else {
            if let Some(platform) = collision_platform {
                if self.player.velocity < 0 {
                    self.player.velocity = 5;
                    self.player.coordinate.y = platform.coordinate.y;
                } else {
                    self.player.velocity -= 1;
                }
            } else {
                self.player.velocity -= 1;
            }
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
    }
}

#[cfg(test)]
mod test {
    use super::Game;
    use crate::game_object::{Coordinate, Platform, PlayerCharacter};

    #[test]
    fn test_jump() {
        let mut game = Game {
            height: 10,
            player: PlayerCharacter::new(),
            width: 10,
            platforms: vec![],
        };
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player.coordinate.x, 1);
        assert_eq!(game.player.coordinate.y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 15);

        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 1);
    }

    #[test]
    fn test_hit_platform() {
        let mut game = Game {
            height: 10,
            player: PlayerCharacter::new(),
            width: 10,
            platforms: vec![Platform {
                length: 3,
                coordinate: Coordinate { x: 1, y: 8 },
            }],
        };
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player.coordinate.y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.velocity, 5);
        assert_eq!(game.player.coordinate.y, 8);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
    }

    #[test]
    fn test_miss_platform() {
        let mut game = Game {
            height: 10,
            player: PlayerCharacter::new(),
            width: 10,
            platforms: vec![Platform {
                length: 3,
                coordinate: Coordinate { x: 3, y: 15 },
            }],
        };
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player.coordinate.y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::MovingRight);
        assert_eq!(game.player.coordinate.x, 2);
        assert_eq!(game.player.coordinate.y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 6);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 1);
    }

    #[test]
    fn test_start_jump() {
        let mut game = Game::new(10, 20);
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player.coordinate.x, 1);
        assert_eq!(game.player.coordinate.y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 15);

        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 1);
        assert_eq!(game.player.velocity, 5);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player.coordinate.y, 16);
    }
}
