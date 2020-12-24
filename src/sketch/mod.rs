mod handler;

pub use handler::Handler;

pub struct Settings<'a> {
    pub title: Option<&'a str>,
    pub size: [f32; 2],
    pub decorations: bool,
}
pub struct Sketch {}
