mod handler;
pub(crate) mod run;
mod sketch;

pub use self::handler::Handler;
pub use self::sketch::Sketch;
use crate::types::{Key, Size};

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
