use std::io::{self, Stdout};
#[cfg(not(target_arch = "wasm32"))]
use termion::input::TermRead;
#[cfg(not(target_arch = "wasm32"))]
use termion::raw::{IntoRawMode, RawTerminal};

#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_terminal() -> (
    RawTerminal<Stdout>,
    termion::input::Keys<termion::AsyncReader>,
) {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = termion::async_stdin().keys();
    (stdout, stdin)
}
