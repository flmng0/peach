mod handler;
pub(crate) mod run;
mod sketch;

use std::time::{Duration, Instant};

pub use self::handler::Handler;
pub use self::sketch::Sketch;
use crate::types::{Key, Size};

#[derive(Copy, Clone)]
pub struct Delta {
    pub(self) last_update_instant: Instant,
    pub(self) last_draw_instant: Instant,
    pub(self) start_instant: Instant,
    pub since_last_update: Duration,
    pub since_last_draw: Duration,
    pub since_start: Duration,
}

impl Delta {
    pub(self) fn new() -> Self {
        let now = Instant::now();

        Self {
            last_update_instant: now,
            last_draw_instant: now,
            start_instant: now,
            since_last_update: Duration::default(),
            since_last_draw: Duration::default(),
            since_start: Duration::default(),
        }
    }

    pub(self) fn update(&mut self) {
        self.since_last_update = self.last_update_instant.elapsed();
        self.since_last_draw = self.last_draw_instant.elapsed();
        self.since_start = self.start_instant.elapsed();
    }
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
