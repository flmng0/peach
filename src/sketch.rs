use crate::{error::SketchError, lifecycle::Handler, state};

use std::time::{Duration, Instant};

use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Graphics {
    draw_state: state::DrawState,
}

pub struct Sketch {
    pub(crate) window: Window,
    frame_delay: Option<Duration>,
    running: bool,
    draw_state: state::DrawState,
}

impl Sketch {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, SketchError> {
        // Initially hide window until setup function has been completed.
        //
        // This way the window doesn't flicker as being the default window
        // size and is only displayed at the correct size.
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)?;

        Ok(Self {
            window,
            frame_delay: Some(Duration::new(1, 0) / 60),
            running: false,
            draw_state: state::DrawState::default(),
        })
    }

    pub fn set_frame_rate(&mut self, fps: Option<u32>) {
        self.frame_delay = fps.map(|fps| Duration::new(1, 0) / fps);
    }

    pub fn get_frame_rate(&self) -> Option<u32> {
        // Unfortunate loss of a lot of data but it's nearly impossible
        // for the delay to be less than `Duration::new(1, 0) / u32::MAX`.
        self.frame_delay
            .map(|delay| (Duration::new(1, 0).as_nanos() / delay.as_nanos()) as u32)
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.window.set_inner_size(LogicalSize::new(width, height));
    }

    pub fn get_size(&self) -> (f32, f32) {
        self.window
            .inner_size()
            .to_logical::<f32>(self.window.scale_factor())
            .into()
    }

    pub fn exit(&mut self) {
        self.running = false;
    }

    pub(crate) fn run<H>(mut self, event_loop: EventLoop<()>, mut handler: H) -> !
    where
        H: 'static + Handler,
    {
        self.running = true;
        self.window.set_visible(true);

        let mut input = state::InputState::from_window(&self.window);
        let mut delta = state::Delta::new();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => handler.quit(),

                Event::MainEventsCleared => {
                    delta.last_update_instant = Instant::now();
                    delta.update();

                    handler.update(&mut self, &input, delta.clone());

                    match self.frame_delay {
                        Some(delay) => {
                            if delta.time_since_last_frame > delay {
                                self.window.request_redraw();
                            }
                        }
                        None => {
                            self.window.request_redraw();
                        }
                    }
                }

                Event::RedrawRequested(..) => {
                    delta.last_frame_instant = Instant::now();

                    let mut gfx = Graphics {
                        draw_state: self.draw_state.clone(),
                    };
                    handler.draw(&mut gfx);

                    // do something we our gfx commands.
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::ModifiersChanged(modifiers) => input.modifiers = modifiers,
                    WindowEvent::Resized(physical) => {
                        let logical = physical.to_logical::<f64>(self.window.scale_factor());

                        input.size = logical.into();
                        handler.resized(&input, logical.into());
                    }
                    WindowEvent::Moved(physical) => {
                        let logical = physical.to_logical::<f64>(self.window.scale_factor());

                        input.position = logical.into();
                        handler.moved(&input, logical.into());
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state,
                                ..
                            },
                        ..
                    } => {
                        if let Some(exit_key) = <H as Handler>::EXIT_KEY {
                            if key == exit_key {
                                *control_flow = ControlFlow::Exit;
                            }
                        }

                        handler.key(&input, key, state);
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        handler.mouse_button(&input, button, state);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let logical = position.to_logical::<f64>(self.window.scale_factor());

                        input.mouse_position = logical.into();
                        handler.mouse_moved(&input, logical.into());
                    }
                    _ => {}
                },
                _ => {}
            };

            if self.running == false {
                *control_flow = ControlFlow::Exit;
            }
        })
    }
}
