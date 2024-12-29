use crate::control::PlayerControl;

#[derive(Debug)]
pub struct Game {
    pub player_pos_x: usize,
    pub player_pos_y: usize,
    pub player_velocity: isize,
    pub width: usize,
    pub height: usize,

    pub platforms: Vec<Platform>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            player_pos_x: 1,
            player_pos_y: 1,
            player_velocity: 5,
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
        }
    }

    pub fn next(&mut self, player_control: &PlayerControl) {
        // check platform collision:
        // FIXME: can we improve the efficiency here? whole loop is not very good
        let collision_platform = self.platforms.iter().find(|platform| {
            let platform_x_values =
                platform.coordinate.x..(platform.coordinate.x + platform.length);
            let player_next_frame_y_collision_range =
                ((self.player_pos_y as isize + self.player_velocity) as usize - 1)
                    ..=self.player_pos_y;

            let platform_in_line = platform_x_values.contains(&self.player_pos_x);
            let platform_in_player_path =
                player_next_frame_y_collision_range.contains(&platform.coordinate.y);
            println!(
                "{}, {}, {}, {}",
                self.player_pos_y, self.player_pos_x, platform_in_line, platform_in_player_path
            );
            platform_in_line && platform_in_player_path
        });

        self.player_pos_y = ((self.player_pos_y as isize + self.player_velocity).max(0) as usize);

        if self.player_pos_y <= 1 {
            self.player_velocity = 5;
        } else {
            if let Some(platform) = collision_platform {
                if self.player_velocity < 0 {
                    self.player_velocity = 5;
                    self.player_pos_y = platform.coordinate.y;
                } else {
                    self.player_velocity -= 1;
                }
            } else {
                self.player_velocity -= 1;
            }
        }
        match player_control {
            PlayerControl::MovingLeft if self.player_pos_x > 0 => {
                self.player_pos_x -= 1;
            }
            PlayerControl::MovingRight if self.player_pos_x < self.width - 1 => {
                self.player_pos_x += 1;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Platform {
    pub coordinate: Coordinate,
    pub length: usize,
}

#[cfg(test)]
mod test {
    use crate::game::{Coordinate, Game, Platform};

    #[test]
    fn test_jump() {
        let mut game = Game {
            height: 10,
            player_pos_x: 1,
            player_pos_y: 1,
            player_velocity: 5,
            width: 10,
            platforms: vec![],
        };
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player_pos_x, 1);
        assert_eq!(game.player_pos_y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 15);

        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 1);
    }

    #[test]
    fn test_hit_platform() {
        let mut game = Game {
            height: 10,
            player_pos_x: 2,
            player_pos_y: 1,
            player_velocity: 5,
            width: 10,
            platforms: vec![Platform {
                length: 3,
                coordinate: Coordinate { x: 1, y: 8 },
            }],
        };
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player_pos_y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_velocity, 5);
        assert_eq!(game.player_pos_y, 8);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 13);
    }

    #[test]
    fn test_miss_platform() {
        let mut game = Game {
            height: 10,
            player_pos_x: 1,
            player_pos_y: 1,
            player_velocity: 5,
            width: 10,
            platforms: vec![Platform {
                length: 3,
                coordinate: Coordinate { x: 2, y: 15 },
            }],
        };
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player_pos_y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::MovingRight);
        assert_eq!(game.player_pos_x, 2);
        assert_eq!(game.player_pos_y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 6);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 1);
    }

    #[test]
    fn test_start_jump() {
        let mut game = Game::new(10, 20);
        game.next(&crate::control::PlayerControl::Still);

        assert_eq!(game.player_pos_x, 1);
        assert_eq!(game.player_pos_y, 6);

        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 10);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 13);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 15);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 15);

        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 1);
        assert_eq!(game.player_velocity, 5);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        game.next(&crate::control::PlayerControl::Still);
        assert_eq!(game.player_pos_y, 16);
    }
}
