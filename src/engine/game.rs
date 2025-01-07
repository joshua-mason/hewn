pub trait BaseGame {
    fn start_game(&mut self);

    fn set_player_control_key(&mut self, key: Option<termion::event::Key>);

    fn next(&mut self);
}
