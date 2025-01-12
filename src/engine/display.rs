use super::game_object::GameObject;
use std::io::Stdout;
use std::io::Write;
use std::iter::zip;
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

    fn levels(&mut self, game_objects: &[Box<dyn GameObject>]) -> Vec<String> {
        let mut level_strings: Vec<String> = vec![];
        for height in 0..self.screen_height() {
            let level = self.render_level(game_objects, height);
            level_strings.push(level);
        }
        level_strings
    }

    fn render_level(&mut self, game_objects: &[Box<dyn GameObject>], height: u16) -> String {
        let mut level: String = build_string('.', self.screen_width() as usize);
        let y_position = self.screen_height() + self.view_cursor() - height;

        for game_object in game_objects {
            let coords = game_object.get_coords();
            let width = game_object.width();
            let mut display_string: &str = &game_object.display();
            if display_string.len() > width {
                display_string = display_string.split_at(width).0;
            }
            if (coords.y == (y_position as usize)) {
                level.replace_range(coords.x..(coords.x + width), &display_string)
            }
        }

        level
    }

    fn player_view(&mut self, game_objects: &[Box<dyn GameObject>]) -> String {
        let levels = self.levels(game_objects);

        let gotos =
            (0..self.screen_height()).map(|height| termion::cursor::Goto(1, height).to_string());
        zip(levels, gotos)
            .map(|(level, goto)| format!("{}{}", level, goto))
            .collect::<String>()
    }
}

pub fn build_string(ch: char, length: usize) -> String {
    ch.to_string().repeat(length)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_build_string() {
        let input = build_string('@', 3);
        assert_eq!(input, "@@@");
    }
}
