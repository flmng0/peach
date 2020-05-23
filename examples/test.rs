use peach::{
    lifecycle::Handler,
    sketch::{Graphics, Sketch},
    state::{Delta, ElementState, InputState, VirtualKeyCode},
};

fn main() {
    peach::run(
        |sketch| {
            sketch.set_size(512.0, 512.0);
        },
        TestHandler,
    );
}

struct TestHandler;

impl Handler for TestHandler {
    const EXIT_KEY: Option<VirtualKeyCode> = Some(VirtualKeyCode::Escape);

    fn draw(&self, _gfx: &mut Graphics) {}
    fn update(&mut self, sketch: &mut Sketch, input: &InputState, _delta: Delta) {
        if input.modifiers.shift() {
            sketch.exit();
        }
    }
}
