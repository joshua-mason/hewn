use hewn::game_object::Coordinate;
use hewn::runtime::{initialize_terminal_io, TerminalRuntime};
use hewn::view::cursor::StaticCursorStrategy;
use hewn::view::{TerminalRenderer, View};

const SCREEN_HEIGHT: u16 = 20;
const SCREEN_WIDTH: u16 = 50;

#[derive(Debug)]
pub struct Player {}

impl Player {
    pub fn new() -> Player {
        Player {}
    }
}

mod game {

    use hewn::{
        ecs::{
            self, Components, Entity, EntityId, PositionComponent, RenderComponent, SizeComponent,
            TrackComponent, VelocityComponent, ECS,
        },
        game::{Entities, GameLogic},
        runtime::Key,
    };

    pub struct MinimalGame {
        entities: Entities,
        started: bool,
        pub ecs: ecs::ECS,
        pub player_entity_id: EntityId,
    }

    impl MinimalGame {
        pub fn new() -> MinimalGame {
            let entities = Entities::new();
            let mut ecs = ecs::ECS::new();
            let player_entity_id = EntityId(0);
            let player_entity = Entity {
                id: player_entity_id,
                components: Components {
                    position_component: Some(PositionComponent { x: 5, y: 5 }),
                    velocity_component: Some(VelocityComponent { x: 0, y: 0 }),
                    render_component: Some(RenderComponent {
                        ascii_character: 'O',
                    }),
                    size_component: Some(SizeComponent { x: 2, y: 2 }),
                    track_component: Some(TrackComponent {}),
                },
            };
            ecs.add_entity(player_entity);

            MinimalGame {
                entities,
                started: false,
                ecs,
                player_entity_id: player_entity_id,
            }
        }

        fn update_player_velocity(&mut self, key: Option<Key>) {
            let player_entity = self.ecs.get_entity_by_id_mut(self.player_entity_id);
            let Some(player_entity) = player_entity else {
                return;
            };
            let Some(velocity) = &mut player_entity.components.velocity_component else {
                return;
            };
            let Some(key) = &key else {
                velocity.x = 0;
                velocity.y = 0;
                return;
            };

            match key {
                Key::Left => {
                    velocity.x = -1;
                    velocity.y = 0;
                }
                Key::Right => {
                    velocity.x = 1;
                    velocity.y = 0;
                }
                Key::Up => {
                    velocity.x = 0;
                    velocity.y = 1;
                }
                Key::Down => {
                    velocity.x = 0;
                    velocity.y = -1;
                }
                _ => {}
            }
        }
    }

    impl GameLogic for MinimalGame {
        fn start_game(&mut self) {
            self.started = true;
        }

        fn next(&mut self, key: Option<Key>) {
            if !self.started {
                return;
            }
            self.update_player_velocity(key);
            self.ecs.step();
        }

        fn entities(&self) -> &Entities {
            &self.entities
        }

        fn ecs(&self) -> &ECS {
            &self.ecs
        }

        fn debug_str(&self) -> Option<String> {
            let Some(player_entity) = self.ecs.get_entity_by_id(self.player_entity_id) else {
                return None;
            };
            let Some(position_component) = &player_entity.components.position_component else {
                return None;
            };

            let start_game_str = if self.started {
                "Started"
            } else {
                "Hit Space to Start"
            };
            Some(format!(
                "Player @ ({}, {}) {}",
                position_component.x, position_component.y, start_game_str
            ))
        }
    }
}

fn main() {
    let (stdout, stdin) = initialize_terminal_io();

    let mut view = View {
        view_cursor: Coordinate { x: 0, y: 0 },
        renderer: Box::new(TerminalRenderer::new(stdout, SCREEN_HEIGHT, SCREEN_WIDTH)),
        cursor_strategy: Box::new(StaticCursorStrategy::new()),
    };

    let mut game = game::MinimalGame::new();
    let mut runtime = TerminalRuntime::new(stdin, &mut game, &mut view);
    runtime.start();
}

#[cfg(test)]
mod test {
    use hewn::{game::GameLogic, runtime::Key};

    use crate::game;

    #[test]
    fn test_player_move() {
        let mut game = game::MinimalGame::new();
        let player = game.ecs.get_entity_by_id(game.player_entity_id);
        assert!(player.is_some());

        game.start_game();
        game.next(Some(Key::Up));

        let player = game.ecs.get_entity_by_id(game.player_entity_id);
        assert!(player.is_some());
        let Some(player_entity) = player else {
            panic!("Player entity not set")
        };
        let Some(position_component) = &player_entity.components.position_component else {
            panic!("Position component not set")
        };
        assert_eq!(position_component.x, 5);
        assert_eq!(position_component.y, 6);
    }
}
