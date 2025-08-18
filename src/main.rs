use asciibird::play_asciibird_in_terminal;
use asciijump::play_asciijump_in_terminal;

mod asciibird;
mod asciijump;
mod engine;

fn main() {
    play_asciijump_in_terminal();
    play_asciibird_in_terminal();
}
