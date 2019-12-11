use crate::{Modifiers, Point, Size};

use std::time::{Duration, Instant};

#[derive(Debug)]
/// Current state of a [`Sketch`][0].
///
/// [0]: ../sketch/struct.Sketch.html
pub struct State {
    /// The current size of the sketch's window.
    pub size: Size,
    /// Position of the window's top-left corner.
    pub window_pos: Point,
    /// Cursor position relative to the upper left corner of
    /// the sketch's window.
    pub cursor: Point,
    /// Active keyboard modifiers.
    pub modifiers: Modifiers,
    /// Current frame count.
    pub frame: usize,
    /// Time that the last frame occurred.
    pub last_frame: Instant,
    /// Time since last frame.
    pub delta: Duration,
}

impl State {
    pub(crate) fn new(size: Size) -> State {
        State {
            size,
            window_pos: Point::zero(),
            cursor: Point::zero(),
            modifiers: Modifiers {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false,
            },
            frame: 0,
            last_frame: Instant::now(),
            delta: Duration::new(0, 0),
        }
    }

    pub(crate) fn resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    pub(crate) fn window_moved(&mut self, new_pos: Point) {
        self.window_pos = new_pos;
    }

    pub(crate) fn mouse_moved(&mut self, new_pos: Point) {
        self.cursor = new_pos;
    }

    pub(crate) fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_frame;
        self.last_frame = now;

        self.frame += 1;
    }
}
