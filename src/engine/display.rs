use std::io::Stdout;

use termion::raw::RawTerminal;

trait BaseDisplay {
    fn new(stdout: RawTerminal<Stdout>, screen_height: u16) -> Self;

    fn stdout(&mut self) -> &mut RawTerminal<Stdout>;

    fn view_cursor(&mut self) -> &mut u16;

    fn screen_height(&mut self) -> &mut u16;

    // TODO abstract Game
    // fn debug(game: &Game) -> String;
}
