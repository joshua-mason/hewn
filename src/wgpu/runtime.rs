use crate::ecs::Entity;
use crate::runtime::GameHandler;
use crate::runtime::Key;
use crate::wgpu::render::State;
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::KeyEvent;
use winit::event::MouseButton;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::Window;

impl TryFrom<winit::keyboard::KeyCode> for Key {
    type Error = &'static str;

    fn try_from(key: winit::keyboard::KeyCode) -> Result<Self, Self::Error> {
        match key {
            winit::keyboard::KeyCode::ArrowLeft => Ok(Key::Left),
            winit::keyboard::KeyCode::ArrowRight => Ok(Key::Right),
            winit::keyboard::KeyCode::ArrowUp => Ok(Key::Up),
            winit::keyboard::KeyCode::ArrowDown => Ok(Key::Down),
            winit::keyboard::KeyCode::Space => Ok(Key::Space),
            winit::keyboard::KeyCode::Escape => Ok(Key::Escape),
            _ => Err("Key not supported"),
        }
    }
}

#[derive(Default)]
pub struct WindowRuntime {}

impl WindowRuntime {
    pub fn new() -> WindowRuntime {
        WindowRuntime::default()
    }

    pub fn with_update_frequency(_update_frequency: u32) -> WindowRuntime {
        WindowRuntime::default()
    }

    pub fn start(&mut self, game: &mut dyn GameHandler) -> anyhow::Result<()> {
        self.start_with_update_frequency(game, 5)
    }

    pub fn start_with_update_frequency(
        &mut self,
        game: &mut dyn GameHandler,
        update_frequency: u32,
    ) -> anyhow::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
        }
        #[cfg(target_arch = "wasm32")]
        {
            console_log::init_with_level(log::Level::Info).unwrap_throw();
        }

        let event_loop = EventLoop::with_user_event().build()?;
        let mut app = App::with_update_frequency(
            #[cfg(target_arch = "wasm32")]
            &event_loop,
            game,
            update_frequency,
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
    pub(crate) frame_counter: u32,
    pub(crate) update_frequency: u32, // Update game every N frames
}

impl<'a> App<'a> {
    pub fn new(
        #[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>,
        game: &'a mut dyn GameHandler,
    ) -> Self {
        Self::with_update_frequency(
            #[cfg(target_arch = "wasm32")]
            event_loop,
            game,
            5,
        )
    }

    pub fn with_update_frequency(
        #[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>,
        game: &'a mut dyn GameHandler,
        update_frequency: u32,
    ) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            render_state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
            game,
            frame_counter: 0,
            update_frequency,
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

        let renderable_entities = self
            .game
            .ecs()
            .get_entities_with_component(crate::ecs::ComponentType::Render)
            .iter()
            .map(|e| **e)
            // probably terrible performance cloning here we when we should pass a reference as we only
            // need to read - but this is a temporary fix for now.
            .collect::<Vec<Entity>>();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::wgpu::render::CameraStrategy;
            self.render_state = Some(
                pollster::block_on(State::new(
                    window,
                    renderable_entities,
                    CameraStrategy::AllEntities,
                ))
                .unwrap(),
            );
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
                self.frame_counter += 1;

                if self.frame_counter >= self.update_frequency {
                    self.game.next();
                    self.frame_counter = 0;
                }

                let renderable_entities = self
                    .game
                    .ecs()
                    .get_entities_with_component(crate::ecs::ComponentType::Render)
                    .iter()
                    .map(|e| **e)
                    // probably terrible performance cloning here we when we should pass a reference as we only
                    // need to read - but this is a quick fix for now.
                    .collect::<Vec<Entity>>();
                state.update(renderable_entities);
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
                let _ = code
                    .try_into()
                    .and_then(|key| Ok(self.game.handle_key(key, key_state.is_pressed())));
            }
            _ => {}
        }
    }
}
