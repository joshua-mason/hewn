use std::io::{self, Stdout};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub fn initialize_terminal() -> (
    RawTerminal<Stdout>,
    termion::input::Keys<termion::AsyncReader>,
) {
    let stdout = io::stdout().into_raw_mode().unwrap();

    let stdin = termion::async_stdin().keys();
    (stdout, stdin)
}
