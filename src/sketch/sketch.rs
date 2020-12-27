pub use super::{Handler, Settings};

use crate::types::{Fullscreen, Key, Modifiers, MouseButton, Point, Size};

use std::collections::HashMap;

use winit::{
    dpi::LogicalSize,
    event::{ElementState, KeyboardInput, WindowEvent},
    window::Window,
};

pub struct Sketch {
    pub(super) window: Window,
    modifiers: Modifiers,
    // renderer: Renderer,
    running: bool,
    pub(super) framerate: Option<u32>,
    pub(super) framerate_dirty: bool,
    mouse_position: Point,
    mouse_buttons: HashMap<MouseButton, bool>,
    keys: HashMap<Key, bool>,
}

impl Sketch {
    pub(super) fn new(window: Window, framerate: Option<u32>) -> Self {
        Self {
            window,
            modifiers: Modifiers::default(),
            running: true,
            framerate,
            framerate_dirty: false,
            mouse_position: Point::zero(),
            mouse_buttons: HashMap::new(),
            keys: HashMap::new(),
        }
    }

    pub(super) fn handle_event<H: Handler>(&mut self, handler: &mut H, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.running = false;
            }
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
                self.keys.insert(key, state == ElementState::Pressed);

                match state {
                    ElementState::Pressed => handler.key_pressed(self, key),
                    ElementState::Released => handler.key_released(self, key),
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let logical = position.to_logical::<f32>(self.window.scale_factor());
                self.mouse_position = Point::new(logical.x, logical.y);

                handler.mouse_moved(self, self.mouse_position);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.mouse_buttons
                    .insert(button, state == ElementState::Pressed);

                match state {
                    ElementState::Pressed => handler.mouse_pressed(self, button),
                    ElementState::Released => handler.mouse_released(self, button),
                }
            }
            _ => {}
        }
    }

    pub(super) fn has_stopped(&self) -> bool {
        !self.running
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
        let scale_factor = self.window.scale_factor();
        let logical_size = physical_size.to_logical(scale_factor);

        Size::new(logical_size.width, logical_size.height)
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
