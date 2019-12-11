use peach::prelude::*;

fn main() {
    peach::run(
        draw,
        Config::default()
            .with_setup(setup)
            .with_exit_key(Key::Escape),
    );
}

fn setup(sketch: &mut Sketch) {
    sketch.clear(0x282a36ff);

    sketch.fill(0xf8f8f2ff);
    sketch.stroke(0x6495edff);
    sketch.stroke_width(20.0);
    sketch.no_stroke();

    sketch.anchor(Anchor::Center);
}

fn draw(sketch: &mut Sketch, state: &State) {
    sketch.rect(state.cursor, Size::new(100.0, 100.0));
}
