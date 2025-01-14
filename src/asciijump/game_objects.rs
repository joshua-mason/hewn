pub mod platform;
pub mod player_character;

#[cfg(test)]
mod test {
    use crate::{
        asciijump::game_objects::{platform, player_character},
        engine::game_object::{utils, Coordinate},
    };

    #[test]
    fn test_velocity_collision() {
        let platform = platform::Platform {
            coordinate: Coordinate { x: 5, y: 5 },
            length: 1,
        };
        let player = player_character::PlayerCharacter {
            coordinate: Coordinate { x: 7, y: 6 },
            velocity: -1,
        };

        assert!(utils::detect_collision(&platform, &player));
    }
}
