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

    sketch.fill(Color::RED);
}

fn draw(sketch: &mut Sketch, state: &State) {
    sketch.temp_clear();
    sketch.translate(state.cursor.to_vector());
    sketch.rotate(45.0);
    sketch.rect(Point::zero(), Size::new(100.0, 100.0));
}
