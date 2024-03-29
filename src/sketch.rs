use std::collections::HashMap;
use std::time::{Duration, Instant};

use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyboardInput, WindowEvent};
use winit::window::Window;

use crate::render::{Graphics, Renderer};
use crate::types::{Color, Fullscreen, Key, Modifiers, MouseButton, Point, Scalar, Size};

#[allow(unused_variables)]
pub trait Handler {
    fn setup(sketch: &mut Sketch) -> Self;
    fn quit(&mut self) {}

    fn draw(&mut self, sketch: &mut Sketch, gfx: &mut Graphics);

    fn key_pressed(&mut self, sketch: &mut Sketch, key: Key) {}
    fn key_released(&mut self, sketch: &mut Sketch, key: Key) {}

    fn mouse_moved(&mut self, sketch: &mut Sketch, position: Point) {}
    fn mouse_pressed(&mut self, sketch: &mut Sketch, button: MouseButton) {}
    fn mouse_released(&mut self, sketch: &mut Sketch, button: MouseButton) {}
}

pub struct Settings<'a> {
    pub title: Option<&'a str>,
    pub size: Size,
    pub decorations: bool,
    pub framerate: Option<u32>,
    pub exit_key: Option<Key>,
}

impl<'a> Default for Settings<'a> {
    fn default() -> Self {
        Self {
            title: None,
            size: Size::new(800.0, 600.0),
            decorations: true,
            framerate: None,
            exit_key: None,
        }
    }
}

pub struct Sketch {
    pub(super) window: Window,
    pub(super) renderer: Renderer,
    pub(super) clear_color: Option<Color>,
    modifiers: Modifiers,
    running: bool,
    pub(super) framerate: Option<u32>,
    pub(super) framerate_dirty: bool,
    start_instant: Instant,
    exit_key: Option<Key>,
    mouse_position: Point,
    mouse_buttons: HashMap<MouseButton, bool>,
    keys: HashMap<Key, bool>,
}

impl Sketch {
    pub(super) fn new(window: Window, settings: Settings) -> Self {
        let renderer = pollster::block_on(Renderer::new(&window)).unwrap();

        Self {
            window,
            renderer,
            clear_color: None,
            modifiers: Modifiers::default(),
            running: true,
            framerate: settings.framerate,
            framerate_dirty: true,
            start_instant: Instant::now(),
            exit_key: settings.exit_key,
            mouse_position: Point::zero(),
            mouse_buttons: HashMap::new(),
            keys: HashMap::new(),
        }
    }

    pub(super) fn handle_event<H: Handler>(&mut self, handler: &mut H, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.running = false;
            },
            WindowEvent::ModifiersChanged(modifiers) => self.modifiers = modifiers,
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(exit_key) = self.exit_key {
                    if exit_key == key {
                        self.running = false;
                    }
                }

                self.keys.insert(key, state == ElementState::Pressed);

                match state {
                    ElementState::Pressed => handler.key_pressed(self, key),
                    ElementState::Released => handler.key_released(self, key),
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Point::new(position.x as Scalar, position.y as Scalar);

                handler.mouse_moved(self, self.mouse_position);
            },
            WindowEvent::MouseInput { button, state, .. } => {
                self.mouse_buttons
                    .insert(button, state == ElementState::Pressed);

                match state {
                    ElementState::Pressed => handler.mouse_pressed(self, button),
                    ElementState::Released => handler.mouse_released(self, button),
                }
            },
            WindowEvent::Resized(size) => {
                self.renderer.resize(size);

                // let logical =
                // size.to_logical(self.window.
                // scale_factor());
                // self.size.width = logical.width;
                // self.size.height = logical.height;
            },
            _ => {},
        }
    }

    pub(super) fn has_stopped(&self) -> bool {
        !self.running
    }

    pub fn get_time_since_start(&self) -> Duration {
        self.start_instant.elapsed()
    }

    pub fn get_key(&self, key: Key) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn get_mouse_button(&self, button: MouseButton) -> bool {
        *self.mouse_buttons.get(&button).unwrap_or(&false)
    }

    pub fn get_mouse_position(&self) -> Point {
        self.mouse_position
    }

    pub fn get_modifiers(&self) -> Modifiers {
        self.modifiers
    }

    pub fn get_center(&self) -> Point {
        let size = self.get_size();
        Point::new(size.width / 2.0, size.height / 2.0)
    }

    pub fn get_size(&self) -> Size {
        let physical_size = self.window.inner_size();
        // let scale_factor = self.window.scale_factor();
        // let logical_size =
        // physical_size.to_logical(scale_factor);

        // Size::new(logical_size.width, logical_size.height)
        Size::new(
            physical_size.width as Scalar,
            physical_size.height as Scalar,
        )
    }

    pub fn get_clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    pub fn set_clear_color<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.clear_color = Some(color.into());
    }

    pub fn no_clear_color<C>(&mut self) {
        self.clear_color = None;
    }

    pub fn set_size(&mut self, new_size: Size) {
        let logical_size = LogicalSize::new(new_size.width, new_size.height);
        self.window.set_inner_size(logical_size);
    }

    pub fn set_framerate(&mut self, framerate: Option<u32>) {
        self.framerate_dirty = true;
        self.framerate = framerate;
    }

    pub fn set_fullscreen(&mut self, fullscreen: Option<Fullscreen>) {
        self.window.set_fullscreen(fullscreen);
    }
}
