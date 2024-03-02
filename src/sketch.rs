use std::collections::HashSet;
use std::time::{Duration, Instant};

use wgpu::SurfaceError;
use winit::dpi::{LogicalSize, PhysicalSize, Size};
use winit::event::{ElementState, Event, KeyboardInput, ModifiersState, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

use crate::prelude::Vector;
use crate::render::{Graphics, RenderError, Renderer};
use crate::types::{Color, Key, MouseButton};

pub trait FromSketch {
    fn from_sketch(sketch: &Sketch) -> Self;
}

impl FromSketch for () {
    fn from_sketch(_sketch: &Sketch) -> Self {
        ()
    }
}

struct Attributes {
    title: &'static str,
    size: Size,
    fullscreen: bool,
    exit_key: Option<Key>,
    clear_color: Option<Color>,
}

impl Attributes {
    const fn new(title: &'static str) -> Self {
        Self {
            title,
            size: Size::Logical(LogicalSize::new(640.0, 480.0)),
            fullscreen: false,
            exit_key: None,
            clear_color: None,
        }
    }
}

pub struct Sketch {
    attr: Attributes,
    scale_factor: f64,
    start: Instant,
    input_state: InputState,
}

impl Sketch {
    pub fn builder(title: &'static str) -> SketchBuilder {
        SketchBuilder::new(title)
    }

    pub fn run<Drawer, Model>(mut self, mut draw: Drawer) -> !
    where
        Drawer: 'static + FnMut(&mut Sketch, &mut Model) -> Graphics,
        Model: 'static + FromSketch,
    {
        let event_loop = EventLoop::new();
        let window = Self::init_window(&event_loop, &self.attr);

        let now = Instant::now();
        self.start = now;
        self.scale_factor = window.scale_factor();

        let mut renderer = pollster::block_on(Renderer::new(&window)).unwrap();

        window.set_visible(true);

        let mut model = Model::from_sketch(&self);

        let mut last_draw = now;
        // TODO: Currently assumes 60FPS.
        let frame_delay = Duration::new(1, 0) / 60;

        event_loop.run(move |event, _, control_flow| {
            // keep
            match event {
                Event::MainEventsCleared => {
                    let delta = last_draw.elapsed();
                    let draw = delta >= frame_delay;

                    if draw {
                        window.request_redraw();
                    }
                },
                Event::RedrawRequested(..) => {
                    let gfx = draw(&mut self, &mut model);

                    if let Err(e) = renderer.render(gfx) {
                        match e {
                            RenderError::SurfaceTexture(e) => {
                                match e {
                                    SurfaceError::Lost => {
                                        renderer.resize(
                                            self.attr.size.to_physical(window.scale_factor()),
                                        )
                                    },
                                    SurfaceError::OutOfMemory => {
                                        eprintln!("Peach: GPU out of memory");
                                        *control_flow = ControlFlow::Exit;
                                    },
                                    e => eprintln!("{:?}", e),
                                }
                            },
                            e => {
                                eprintln!("{:?}", e);
                                *control_flow = ControlFlow::Exit;
                            },
                        }
                    }

                    last_draw = Instant::now();
                },
                Event::WindowEvent { event, .. } => {
                    if !self.input_state.handle_event(&event) {
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            _ => {},
                        }
                    }

                    if let Some(exit_key) = self.attr.exit_key {
                        if self.input_state.keys.contains(&exit_key) {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                },
                _ => {},
            }
        })
    }

    pub fn secs_since_start(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    pub fn corner_br(&self) -> Vector {
        self.size()
    }

    pub fn mouse(&self) -> Vector {
        self.input_state.mouse_position
    }

    pub fn size(&self) -> Vector {
        let physical: PhysicalSize<u32> = self.attr.size.to_physical(self.scale_factor);

        Vector::new(physical.width as _, physical.height as _)
    }

    pub fn center(&self) -> Vector {
        self.size() / 2.0
    }

    pub fn new_graphics(&self) -> Graphics {
        Graphics::new(self.attr.clear_color)
    }

    // Eventually return Result<Window, SketchInitError>
    fn init_window(event_loop: &EventLoop<()>, attr: &Attributes) -> Window {
        let fullscreen = attr.fullscreen.then(|| Fullscreen::Borderless(None));

        WindowBuilder::new()
            .with_title(attr.title)
            .with_inner_size(attr.size)
            .with_fullscreen(fullscreen)
            .with_visible(false)
            .build(event_loop)
            .unwrap() // TODO: Handle error
    }

    fn from_builder(builder: SketchBuilder) -> Self {
        Self {
            attr: builder.attr,
            input_state: InputState::default(),
            scale_factor: 1.0,
            start: Instant::now(),
        }
    }
}

pub struct SketchBuilder {
    attr: Attributes,
}

impl SketchBuilder {
    pub const fn new(title: &'static str) -> Self {
        Self {
            attr: Attributes::new(title),
        }
    }

    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.attr.size = Size::Logical(LogicalSize::new(width, height));
        self
    }

    pub fn physical_size(mut self, width: u32, height: u32) -> Self {
        self.attr.size = Size::Physical(PhysicalSize::new(width, height));
        self
    }

    pub fn clear_color(mut self, color: Color) -> Self {
        self.attr.clear_color = Some(color);
        self
    }

    pub fn fullscreen(mut self) -> Self {
        self.attr.fullscreen = true;
        self
    }

    pub fn exit_key(mut self, key: Key) -> Self {
        self.attr.exit_key = Some(key);
        self
    }

    // Eventually return Result<Sketch, SketchInitError>
    pub fn build(self) -> Sketch {
        Sketch::from_builder(self)
    }
}

#[derive(Default, Debug, Clone)]
pub struct InputState {
    keys: HashSet<Key>,
    modifiers: ModifiersState,
    buttons: HashSet<MouseButton>,
    mouse_position: Vector,
}

impl InputState {
    // Returns whether the event was handled.
    fn handle_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => {
                match state {
                    ElementState::Pressed => self.keys.insert(*key),
                    ElementState::Released => self.keys.remove(key),
                };

                true
            },
            WindowEvent::MouseInput { button, state, .. } => {
                match state {
                    ElementState::Pressed => self.buttons.insert(*button),
                    ElementState::Released => self.buttons.remove(button),
                };

                true
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position.x = position.x;
                self.mouse_position.y = position.y;

                true
            },
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = *modifiers;

                true
            },
            _ => false,
        }
    }
}
