use peach::prelude::*;

fn main() {
    peach::run(
        draw,
        Config::default()
            .with_framerate(1)
            .with_exit_key(Key::Escape),
    );
}

fn draw(sketch: &mut Sketch, state: &State) {
    dbg!(state);
}
