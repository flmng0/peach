use std::collections::HashMap;

use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyboardInput, WindowEvent};
use winit::window::Window;

pub use super::{Handler, Settings};
use crate::render::Renderer;
use crate::types::{Color, Fullscreen, Key, Modifiers, MouseButton, Point, Scalar, Size};

pub struct Sketch {
    pub(super) window: Window,
    pub(super) renderer: Renderer,
    pub(super) clear_color: Option<Color>,
    size: Size,
    modifiers: Modifiers,
    running: bool,
    pub(super) framerate: Option<u32>,
    pub(super) framerate_dirty: bool,
    exit_key: Option<Key>,
    mouse_position: Point,
    mouse_buttons: HashMap<MouseButton, bool>,
    keys: HashMap<Key, bool>,
}

impl Sketch {
    pub(super) fn new(window: Window, settings: Settings) -> Self {
        let size = window.inner_size().to_logical(window.scale_factor());
        let renderer = futures::executor::block_on(Renderer::new(&window)).unwrap();

        Self {
            window,
            renderer,
            clear_color: None,
            size: Size::new(size.width, size.height),
            modifiers: Modifiers::default(),
            running: true,
            framerate: settings.framerate,
            framerate_dirty: false,
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

                self.size.width = size.width as Scalar;
                self.size.height = size.height as Scalar;
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

    pub fn center(&self) -> Point {
        Point::new(self.size.width / 2.0, self.size.height / 2.0)
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
