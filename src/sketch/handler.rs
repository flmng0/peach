use super::Sketch;
use crate::graphics::Graphics;

pub trait Handler {
    fn setup(sketch: &mut Sketch) -> Self;

    fn draw(&self, sketch: &mut Sketch, gfx: &mut Graphics);
}
