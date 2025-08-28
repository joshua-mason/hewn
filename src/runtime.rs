//! Wasm and terminal game runtimes.

use super::view::View;
use crate::ecs::ComponentType;
use crate::ecs::Entity;
use crate::ecs::ECS;
use crate::render::wgpu::State;
#[cfg(not(target_arch = "wasm32"))]
use std::io::{self, Stdout};
use std::sync::Arc;
use std::{
    thread,
    time::{self, Duration, Instant},
};
#[cfg(not(target_arch = "wasm32"))]
use termion::input::TermRead;
#[cfg(not(target_arch = "wasm32"))]
use termion::raw::{IntoRawMode, RawTerminal};
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::KeyEvent;
use winit::event::MouseButton;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::Window;

const FRAME_RATE_MILLIS: u64 = 10;
const GAME_STEP_MILLIS: u64 = 100;

/// Trait which all games must implement.
///
/// TODO rename to GameHandler to better conform to other naming conventions? e.g. winit app handler.
pub trait GameHandler {
    /// Start the game.
    fn start_game(&mut self);
    /// Compute the next game state based on player input.
    fn next(&mut self);
    /// Get the Entity Component System
    fn ecs(&self) -> &ECS;

    /// Get a string for debugging.
    fn debug_str(&self) -> Option<String>;

    fn handle_key(&mut self, key: Key, pressed: bool) -> bool;
}

/// Key for player control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}

impl From<winit::keyboard::KeyCode> for Key {
    fn from(key: winit::keyboard::KeyCode) -> Self {
        match key {
            winit::keyboard::KeyCode::ArrowLeft => Key::Left,
            winit::keyboard::KeyCode::ArrowRight => Key::Right,
            winit::keyboard::KeyCode::ArrowUp => Key::Up,
            winit::keyboard::KeyCode::ArrowDown => Key::Down,
            winit::keyboard::KeyCode::Space => Key::Space,
            winit::keyboard::KeyCode::Escape => Key::Escape,
            _ => panic!("Unsupported key: {:?}", key),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<termion::event::Key> for Key {
    fn from(key: termion::event::Key) -> Self {
        match key {
            termion::event::Key::Left => Key::Left,
            termion::event::Key::Right => Key::Right,
            termion::event::Key::Up => Key::Up,
            termion::event::Key::Down => Key::Down,
            termion::event::Key::Char(' ') => Key::Space,
            termion::event::Key::Esc => Key::Escape,
            _ => panic!("Unsupported key: {:?}", key),
        }
    }
}

/// Initialize terminal IO.
#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_terminal_io() -> (
    RawTerminal<Stdout>,
    termion::input::Keys<termion::AsyncReader>,
) {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = termion::async_stdin().keys();
    (stdout, stdin)
}

/// A runtime for a terminal game.
#[cfg(not(target_arch = "wasm32"))]
pub struct TerminalRuntime {
    pub stdin: termion::input::Keys<termion::AsyncReader>,
    pub display: View,
    last_frame_time: Instant,
    player_control_key: Option<Key>,
}

#[cfg(not(target_arch = "wasm32"))]
impl TerminalRuntime {
    pub fn new(width: u16, height: u16) -> TerminalRuntime {
        use crate::view::{
            cursor::FollowPlayerYCursorStrategy, ScreenDimensions, TerminalRenderer, ViewCoordinate,
        };

        let (stdout, stdin) = initialize_terminal_io();

        let view = View {
            view_cursor: ViewCoordinate { x: 0, y: 0 },
            renderer: Box::new(TerminalRenderer::new(
                stdout,
                ScreenDimensions {
                    x: width,
                    y: height,
                },
            )),
            cursor_strategy: Box::new(FollowPlayerYCursorStrategy::new()),
        };

        TerminalRuntime {
            stdin,
            last_frame_time: Instant::now(),
            display: view,
            player_control_key: None,
        }
    }

    /// Start the game loop listening for player input and rendering the game.
    pub fn start(&mut self, game: &mut dyn GameHandler) {
        loop {
            use crate::ecs::ComponentType;

            let input = self.stdin.next();

            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Char('q') => break,
                    key if key != termion::event::Key::Char(' ') => {
                        game.handle_key(key.into(), true);
                    }
                    termion::event::Key::Char(' ') => {
                        game.start_game();
                    }
                    _ => {
                        self.player_control_key = None;
                    }
                }
            }
            thread::sleep(time::Duration::from_millis(FRAME_RATE_MILLIS));

            let now = time::Instant::now();
            if now - self.last_frame_time > Duration::from_millis(GAME_STEP_MILLIS) {
                game.next();
                self.last_frame_time = now;

                if input.is_none() {
                    self.player_control_key = None;
                }
            }
            let ecs = game.ecs();
            let entities = ecs.get_entities_by(ComponentType::Render);
            self.display.next(entities, game.debug_str());
        }
    }
}

#[derive(Default)]
pub struct WindowRuntime {}

impl WindowRuntime {
    pub fn new() -> WindowRuntime {
        WindowRuntime::default()
    }

    pub fn start(&mut self, game: &mut dyn GameHandler) -> anyhow::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
        }
        #[cfg(target_arch = "wasm32")]
        {
            console_log::init_with_level(log::Level::Info).unwrap_throw();
        }

        let event_loop = EventLoop::with_user_event().build()?;
        let mut app = App::new(
            #[cfg(target_arch = "wasm32")]
            &event_loop,
            game,
        );
        event_loop.run_app(&mut app)?;

        Ok(())
    }
}

pub struct App<'a> {
    #[cfg(target_arch = "wasm32")]
    pub(crate) proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    pub(crate) render_state: Option<State>,
    pub(crate) game: &'a mut dyn GameHandler,
}

impl<'a> App<'a> {
    pub fn new(
        #[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>,
        game: &'a mut dyn GameHandler,
    ) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            render_state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
            game,
        }
    }
}

impl<'a> ApplicationHandler<State> for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let game_entities = self
            .game
            .ecs()
            .get_entities_by(crate::ecs::ComponentType::Render)
            .iter()
            .map(|e| **e)
            // probably terrible performance cloning here we when we should pass a reference as we only
            // need to read - but this is a quick fix for now.
            .collect::<Vec<Entity>>();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the
            self.render_state =
                Some(pollster::block_on(State::new(window, game_entities)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(proxy
                        .send_event(
                            State::new(window, game_entities)
                                .await
                                .expect("Unable to create canvas!!!")
                        )
                        .is_ok())
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.render_state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.render_state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                // game should move to state?
                self.game.next();
                let game_entities = self
                    .game
                    .ecs()
                    .get_entities_by(crate::ecs::ComponentType::Render)
                    .iter()
                    .map(|e| **e)
                    // probably terrible performance cloning here we when we should pass a reference as we only
                    // need to read - but this is a quick fix for now.
                    .collect::<Vec<Entity>>();
                state.update(game_entities);
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {}
                (MouseButton::Left, false) => {}
                _ => {}
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                state.handle_key(event_loop, code, key_state.is_pressed());
                self.game.handle_key(code.into(), key_state.is_pressed());
            }
            _ => {}
        }
    }
}

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen(start)]
// pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
//     console_error_panic_hook::set_once();
//     run().unwrap_throw();

//     Ok(())
// }

// ------------------------------------------------------------------------------------------------

/// A runtime for a web game.
pub struct WebRuntime {
    game: Box<dyn GameHandler>,
    display: View,
}

impl WebRuntime {
    /// Create a new web runtime.
    pub fn new(game: Box<dyn GameHandler>, display: View) -> WebRuntime {
        WebRuntime { game, display }
    }

    /// Start the game loop.
    pub fn start(&mut self) {
        self.game.start_game();
    }

    /// Compute the next game state based on player input.
    pub fn tick(&mut self) {
        self.game.next();
    }

    /// Render the game to a string.
    pub fn render(&mut self) -> String {
        let ecs = self.game.ecs();
        let entities = ecs.get_entities_by(ComponentType::Render);
        self.display.next(entities, self.game.debug_str())
    }
}

/// A web game API.
#[wasm_bindgen]
pub struct WasmGameApi {
    web_runtime: WebRuntime,
}

#[wasm_bindgen]
impl WasmGameApi {
    pub fn start(&mut self) {
        self.web_runtime.start();
    }

    pub fn tick(&mut self) {
        self.web_runtime.tick();
    }

    pub fn render(&mut self) -> String {
        self.web_runtime.render()
    }
}

pub fn new_wasm_game_api(web_runtime: WebRuntime) -> WasmGameApi {
    WasmGameApi { web_runtime }
}

#[wasm_bindgen]
pub enum WasmKey {
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
}
