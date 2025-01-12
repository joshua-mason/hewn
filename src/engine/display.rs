use super::game_object::GameObject;
use std::io::Stdout;
use std::io::Write;
use termion::raw::RawTerminal;

pub trait BaseDisplay {
    fn stdout(&mut self) -> &mut RawTerminal<Stdout>;

    fn view_cursor(&self) -> u16;

    fn next(&mut self, game_objects: &[Box<dyn GameObject>], debug_string: Option<String>) {
        self.update_cursor(game_objects);
        let view = self.player_view(game_objects);
        let h: u16 = self.screen_height();
        write!(
            self.stdout(),
            "{}{}{}{}{:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            view,
            termion::cursor::Goto(1, h + 2),
            debug_string
        )
        .unwrap();

        self.stdout().lock().flush().unwrap();
    }

    fn update_cursor(&mut self, game_objects: &[Box<dyn GameObject>]);

    fn screen_height(&self) -> u16;

    fn screen_width(&self) -> u16;

    fn player_view(&mut self, game_objects: &[Box<dyn GameObject>]) -> String;
}
