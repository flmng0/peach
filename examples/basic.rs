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
}

fn draw(sketch: &mut Sketch, state: &State) {
    sketch.background(0x282a36ff.into());
    let frame = state.frame as f32;

    sketch.fill(Color::hsl(state.frame as f32, 1.0, 0.5));

    let (cx, cy) = state.cursor.into();

    sketch.translate(cx, cy);
    sketch.rotate(state.frame as f32);

    let size = 50.0 + ((frame / 60.0).sin() + 1.0) * 50.0;

    sketch.rect(0.0, 0.0, size, size);
}
