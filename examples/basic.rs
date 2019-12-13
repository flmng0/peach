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
    sketch.anchor(Anchor::Center);

    sketch.no_stroke();
    sketch.fill(0x282a36ff);
}

fn draw(sketch: &mut Sketch, state: &State) {
    sketch.clear(Color::hsl(state.frame as f32, 1.0, 0.5));

    let (cx, cy) = state.cursor.into();

    sketch.rotate(45.0);
    sketch.translate(cx, cy);
    sketch.rect(0.0, 0.0, 100.0, 100.0);
}
