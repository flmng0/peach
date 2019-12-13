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

    sketch.clear(0x282a36ff);
    sketch.fill(Color::RED);
}

fn draw(sketch: &mut Sketch, state: &State) {
    sketch.push();

    sketch.fill(Color::BLUE);
    sketch.rotate(45.0);
    sketch.translate(state.cursor.to_vector());
    sketch.rect(Point::zero(), (100.0, 100.0).into());

    sketch.pop();

    sketch.translate(state.cursor.to_vector());
    sketch.rect(Point::zero(), (100.0, 100.0).into());
}
