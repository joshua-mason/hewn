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
