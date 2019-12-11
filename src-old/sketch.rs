use crate::{Renderer, RendererError};

use std::time::{Duration, Instant};

use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    error::OsError,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Debug)]
/// TODO: Document `SketchError`
pub enum SketchError {
    WindowCreateError(OsError),
    RendererCreateError(RendererError),
}

impl From<RendererError> for SketchError {
    fn from(err: RendererError) -> Self {
        SketchError::RendererCreateError(err)
    }
}

/// Configuration for a [`Sketch`].
///
/// # Defaults
/// ```
/// Config {
///     name: String::from("peach sketch"),
///     width: 640,
///     height: 480,
///     framerate: None,
/// }
/// ```
///
/// [`Sketch`]: struct.Sketch.html
#[derive(Debug)]
pub struct Config {
    /// Name of the sketch, which will be used for the
    /// window's title.
    pub name: String,
    /// Width of the sketch's window.
    pub width: u32,
    /// Height of the sketch's window.
    pub height: u32,
    /// Framerate, or frames per second, of the sketch. Set
    /// to `None` for an un-capped framerate.
    pub framerate: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::from("peach sketch"),
            width: 640,
            height: 480,
            framerate: None,
        }
    }
}

/// Represents the current state of a sketch, and can be
/// used to retrieve values, such as keyboard state,
/// frame count, etc.
///
/// # Usage
/// A `State` is passed to all methods of a [`Handler`]
/// by value:
///
/// ```no_run
/// struct Example {
///     x: f32,
/// }
///
/// impl Handler for Example {
///     fn update(&mut self, state: State) {
///         self.x += state.delta.as_secs_f32();
///     }
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct State {
    /// Frame count.
    pub frame: usize,

    /// Delta time since last update.
    pub delta: Duration,

    /// Position of the mouse cursor.
    pub cursor: PhysicalPosition,
}

/// Represents a sketch.
///
/// # Usage
/// TODO: Usage snippet for a `Sketch`
pub struct Sketch {
    event_loop: EventLoop<()>,
    window: Window,
    renderer: Renderer,
}

impl Sketch {
    pub fn new(config: Config) -> Result<Sketch, SketchError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(config.name)
            .with_inner_size(LogicalSize::new(config.width as _, config.height as _))
            .build(&event_loop)
            .or_else(|err| Err(SketchError::WindowCreateError(err)))?;

        let renderer = Renderer::new(&window)?;

        Ok(Sketch {
            event_loop,
            window,
            renderer,
        })
    }

    pub fn run<H: 'static + Handler>(self, mut handler: H) -> ! {
        let Self {
            event_loop,
            window,
            mut renderer,
        } = self;

        let mut last_time = Instant::now();
        let mut state = State {
            frame: 0,
            delta: Duration::new(0, 0),
            cursor: PhysicalPosition::new(0.0, 0.0),
        };

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::EventsCleared => {
                    state.frame += 1;

                    let now = Instant::now();
                    state.delta = now - last_time;
                    last_time = now;

                    handler.update(&state);

                    window.request_redraw();
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(logical) => {
                        let physical = logical.to_physical(window.hidpi_factor());

                        renderer.resize(physical);
                    },
                    WindowEvent::RedrawRequested => {
                        handler.draw(&state, &mut renderer);
                        renderer.finish();
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        state.cursor = position.to_physical(window.hidpi_factor());
                    },
                    // TODO: Handle all events, remove hard-coded escape-quit.
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    }
                    | WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    _ => {},
                },
                _ => {},
            }
        })
    }
}

// TODO: Document `Handler`.
pub trait Handler {
    fn update(&mut self, state: &State);

    fn draw(&mut self, state: &State, renderer: &mut Renderer);

    // TODO: Event callbacks for `Handler`.
    // fn resized(&mut self, state: State, old: LogicalSize,
    // new: LogicalSize);
}
