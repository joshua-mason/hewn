use asciibird::play_asciibird_in_terminal;
use asciijump::play_asciijump_in_terminal;
use snake::play_snake_in_terminal;

mod asciibird;
mod asciijump;
mod engine;
mod snake;

fn main() {
    play_snake_in_terminal();
    play_asciijump_in_terminal();
    play_asciibird_in_terminal();
}
