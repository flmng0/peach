use crate::{color, tess};

use std::time::{Duration, Instant};

pub use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, ModifiersState, MouseButton, VirtualKeyCode},
    window::Window,
};

#[derive(Debug, Clone)]
pub struct Delta {
    pub start_instant: Instant,
    pub time_since_start: Duration,

    pub last_update_instant: Instant,
    pub time_since_last_update: Duration,

    pub last_frame_instant: Instant,
    pub time_since_last_frame: Duration,
}

impl Delta {
    pub(crate) fn new() -> Self {
        let now = Instant::now();

        Self {
            start_instant: now,
            time_since_start: Duration::new(0, 0),

            last_update_instant: now,
            time_since_last_update: Duration::new(0, 0),

            last_frame_instant: now,
            time_since_last_frame: Duration::new(0, 0),
        }
    }

    pub(crate) fn update(&mut self) {
        // To sync each time_since_* with each other properly.
        let now = Instant::now();

        self.time_since_start = now.duration_since(self.start_instant);
        self.time_since_last_update = now.duration_since(self.last_update_instant);
        self.time_since_last_frame = now.duration_since(self.last_frame_instant);
    }
}

#[derive(Debug)]
pub struct InputState {
    pub size: (f64, f64),
    pub position: (f64, f64),
    pub mouse_position: (f64, f64),
    pub modifiers: ModifiersState,
}

impl InputState {
    pub(crate) fn from_window(window: &Window) -> Self {
        let scale_factor = window.scale_factor();
        let size = window.inner_size().to_logical::<f64>(scale_factor).into();
        let position = window
            .inner_position()
            .unwrap_or(PhysicalPosition::new(0, 0))
            .to_logical::<f64>(scale_factor)
            .into();

        Self {
            size,
            position,
            mouse_position: (0.0, 0.0),
            modifiers: ModifiersState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DrawState {
    pub(crate) fill_color: Option<color::Srgba>,
    pub(crate) fill_options: tess::FillOptions,
    pub(crate) stroke_color: Option<color::Srgba>,
    pub(crate) stroke_options: tess::StrokeOptions,
}

impl Default for DrawState {
    fn default() -> Self {
        Self {
            fill_color: Some(color::Srgba::new(1.0, 1.0, 1.0, 1.0)),
            fill_options: tess::FillOptions::default(),
            stroke_color: Some(color::Srgba::new(0.0, 0.0, 0.0, 1.0)),
            stroke_options: tess::StrokeOptions::default(),
        }
    }
}
