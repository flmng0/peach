use crate::Point;

use std::time::Duration;
use winit::{event::ModifiersState, window::Window};

pub struct State {
    pub cursor: Point,
    pub width: f32,
    pub height: f32,
    pub delta: Duration,
    pub modifiers: ModifiersState,
}

impl State {
    pub(crate) fn new(width: f32, height: f32) -> State {
        State {
            cursor: Point::zero(),
            width,
            height,
            delta: Duration::new(0, 0),
            modifiers: ModifiersState {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false,
            },
        }
    }
}

pub struct Sketch {}

impl Sketch {
    pub(crate) fn new(_window: &Window) -> Self {
        Self {}
    }
}
