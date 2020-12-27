use super::{Delta, Sketch};

use crate::{
    render::Graphics,
    types::{Key, MouseButton, Point},
};

#[allow(unused_variables)]
pub trait Handler {
    fn setup(sketch: &mut Sketch) -> Self;
    fn quit(&mut self) {}

    fn update(&mut self, sketch: &mut Sketch, delta: Delta);
    fn draw(&self, sketch: &mut Sketch, gfx: &mut Graphics);

    fn key_pressed(&mut self, sketch: &mut Sketch, key: Key) {}
    fn key_released(&mut self, sketch: &mut Sketch, key: Key) {}

    fn mouse_moved(&mut self, sketch: &mut Sketch, position: Point) {}
    fn mouse_pressed(&mut self, sketch: &mut Sketch, button: MouseButton) {}
    fn mouse_released(&mut self, sketch: &mut Sketch, button: MouseButton) {}
}
