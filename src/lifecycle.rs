use crate::sketch::{Graphics, Sketch};
use crate::state::{Delta, ElementState, InputState, MouseButton, VirtualKeyCode};

use winit::event_loop::EventLoop;

#[allow(unused_variables)]
pub trait Handler {
    const EXIT_KEY: Option<VirtualKeyCode> = None;

    fn draw(&self, gfx: &mut Graphics);
    /// Whether the sketch should redraw after this frame.
    fn update(&mut self, sketch: &mut Sketch, input: &InputState, delta: Delta);

    fn quit(&mut self) {}

    fn resized(&mut self, input: &InputState, size: (f64, f64)) {}
    fn moved(&mut self, input: &InputState, position: (f64, f64)) {}

    fn key(&mut self, input: &InputState, key: VirtualKeyCode, state: ElementState) {}
    fn mouse_button(&mut self, input: &InputState, button: MouseButton, state: ElementState) {}
    fn mouse_moved(&mut self, input: &InputState, position: (f64, f64)) {}
}

pub fn run<S, H>(setup: S) -> !
where
    S: FnOnce(&mut Sketch) -> H,
    H: 'static + Handler,
{
    let event_loop = EventLoop::new();

    let mut sketch = Sketch::new(&event_loop).expect("Unable to initialize sketch");
    let handler = setup(&mut sketch);

    sketch.run(event_loop, handler)
}
