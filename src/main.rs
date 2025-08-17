use asciibird::play_asciibird;
use asciijump::play_asciijump;
use snake::play_snake_in_terminal;

mod asciibird;
mod asciijump;
mod engine;
mod snake;

fn main() {
    // play_snake_in_terminal();
    play_asciijump();
    play_asciibird();
}
