use asciijump::game::{Game, SCREEN_HEIGHT, SCREEN_WIDTH};
use hewn::scene::ComponentType;
use hewn::terminal::render::cursor::FollowPlayerXYCursorStrategy;
use hewn::terminal::render::{Renderer, ScreenDimensions, View, ViewCoordinate};

/// A minimal renderer for tests: no termion control codes, just a newline-separated grid.
struct StringRenderer {
    dims: ScreenDimensions,
}

impl StringRenderer {
    fn new(width: u16, height: u16) -> Self {
        Self {
            dims: ScreenDimensions {
                x: width,
                y: height,
            },
        }
    }
}

impl Renderer for StringRenderer {
    fn screen_height(&self) -> u16 {
        self.dims.y
    }

    fn screen_width(&self) -> u16 {
        self.dims.x
    }

    fn player_view(&mut self, levels: Vec<String>) -> String {
        levels.join("\n")
    }

    fn render(&mut self, _debug_string: Option<String>, view: String, _h: u16) -> String {
        view
    }
}

fn new_test_view() -> View {
    View {
        view_cursor: ViewCoordinate { x: 0, y: 0 },
        renderer: Box::new(StringRenderer::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
        cursor_strategy: Box::new(FollowPlayerXYCursorStrategy::new()),
    }
}

#[test]
fn asciijump_renders_player_character_on_small_screen() {
    // Regression test: with SCREEN_WIDTH=10, the cursor strategy used by TerminalRuntime
    // must not push view_cursor.x negative (which can cause all entities to fail bounds checks).
    let mut game = Game::new(10.0, 20.0, Some(123));
    game.initialise_player();

    let entities = game
        .scene()
        .get_entities_with_component(ComponentType::Render);

    let mut view = new_test_view();
    let rendered = view.next(entities, game.debug_str());

    assert!(
        view.view_cursor.x >= 0,
        "expected non-negative cursor.x, got {}",
        view.view_cursor.x
    );
    assert!(
        rendered.contains('#'),
        "expected to render the player '#', got:\n{}",
        rendered
    );
}

#[test]
fn asciijump_renders_platform_span() {
    let mut game = Game::new(10.0, 20.0, Some(123));
    game.initialise_player();
    // Put a deterministic platform within the initial view.
    game.add_platforms_from_positions(vec![(0.0, 2.0)]);

    let entities = game
        .scene()
        .get_entities_with_component(ComponentType::Render);

    let mut view = new_test_view();
    let rendered = view.next(entities, game.debug_str());

    assert!(
        rendered.contains("==="),
        "expected to render a platform '===', got:\n{}",
        rendered
    );
}
