# Peach
[Processing](https://processing.org)-esque sandboxing library for Rust with [wgpu](https://github.com/gfx-rs/wgpu-rs).

# Example
See the [examples](examples/) folder for more.
```rust
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
    sketch.clear(Color::WHITE);

    sketch.fill(Color::RED);
    sketch.no_stroke();

    sketch.anchor(Anchor::Center);
}

fn draw(sketch: &mut Sketch, state: &State) {
    sketch.translate(state.cursor);
    sketch.rotate(state.frame as f32 / 100.0);
    sketch.rect(Point::zero(), Size::new(100.0, 100.0));
}
```
