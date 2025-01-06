//     pub trait GameObject {

//     }

//     pub trait Game<GameObject> {
//     pub fn new(width: usize, height: usize) -> Game {
//         Game {
//             width,
//             height,
//             state: GameState::Menu,
//             score: 0,
//             game_objects: vec![],
//             player_control_key: None,
//         }
//     }

//     pub fn add_game_objects(&mut self, mut game_objects: Vec<GameObject>) {
//         self.game_objects.append(&mut game_objects);
//     }

//     pub fn next(&mut self) {
//         if self.state != GameState::InGame {
//             return;
//         }
//         self.move_player();
//         self.game_objects.iter_mut().for_each(|o| o.next_step());

//         collision_pass(&mut self.game_objects);

//         // FIXME: can we improve the efficiency here? whole loop is not very good
//         // FIXME: when two platforms, we don't definitely hit the closest one

//         if self.get_player_object().unwrap().velocity < -6 {
//             self.end_game();
//         }

//         self.score = self
//             .score
//             .max(self.get_player_object().unwrap().coordinate.y);
//     }

//     fn move_player(&mut self) {
//         let width = self.width;
//         match self.player_control_key {
//             Some(Key::Left) => {
//                 if let Some(player) = self.get_mut_player_object() {
//                     if player.coordinate.x > 0 {
//                         player.move_left()
//                     }
//                 }
//             }
//             Some(Key::Right) => {
//                 if let Some(player) = self.get_mut_player_object() {
//                     if player.coordinate.x < width - 1 {
//                         player.move_right()
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }

//     pub fn start_game(&mut self) {
//         self.score = 0;
//         self.get_mut_player_object().unwrap().reset();
//         self.state = GameState::InGame;
//     }

//     pub fn end_game(&mut self) {
//         self.state = GameState::Lost(self.score);
//     }

//     pub fn get_player_object(&self) -> Option<&PlayerCharacter> {
//         let player_object = self
//             .game_objects
//             .iter()
//             .find(|o| match o {
//                 GameObject::PlayerCharacter(_) => true,
//                 GameObject::Platform(_) => false,
//             })
//             .unwrap();

//         match player_object {
//             GameObject::PlayerCharacter(player_character) => Some(player_character),
//             GameObject::Platform(_) => None,
//         }
//     }

//     pub fn get_mut_player_object(&mut self) -> Option<&mut PlayerCharacter> {
//         let player_object = self
//             .game_objects
//             .iter_mut()
//             .find(|o| match o {
//                 GameObject::PlayerCharacter(_) => true,
//                 GameObject::Platform(_) => false,
//             })
//             .unwrap();

//         match player_object {
//             GameObject::PlayerCharacter(player_character) => Some(player_character),
//             GameObject::Platform(_) => None,
//         }
//     }

//     pub fn get_platforms(&self) -> Vec<&GameObject> {
//         self.game_objects
//             .iter()
//             .filter(|o| match o {
//                 GameObject::PlayerCharacter(_) => false,
//                 GameObject::Platform(_) => true,
//             })
//             .collect::<Vec<&GameObject>>()
//     }

//     fn set_platforms(&mut self, platforms: Vec<Platform>) {
//         let game_objects = platforms
//             .into_iter()
//             .map(GameObject::Platform)
//             .collect::<Vec<_>>();
//         self.add_game_objects(game_objects);
//     }

//     pub(crate) fn set_player_control_key(&mut self, key: Option<termion::event::Key>) {
//         self.player_control_key = key
//     }
// }
