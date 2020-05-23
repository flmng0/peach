# Peach
[Processing](https://processing.org)-esque sandboxing library for Rust with [wgpu](https://github.com/gfx-rs/wgpu-rs).

# Example
See the [examples](examples/) folder for more code snippets, as well as screen shots.
```rust
use peach::prelude::*;

fn main() {
    peach::run(|sketch| {
        sketch.set_size(512.0, 512.0);
        sketch.fill(Color::hex(0xff4488ff));
        sketch.no_stroke();
    }, Example::default());
}

#[derive(Default)]
struct Example {
    x: f32,
}

impl Handler for Example {
    fn update(&mut self, sketch: &mut Sketch, delta: Delta) {
        self.x = delta.time_since_start.as_secs_f32().sin();
    }

    fn draw(&self, sketch: &mut Sketch) {
        let center = sketch.center();

        sketch.scope(|sketch| {
            sketch.anchorMode(AnchorMode::Center);
            sketch.translate(center.x + self.x * 200.0, center.y);
            sketch.rotate(Angle::Radians(self.x * PI));
            sketch.square(0.0, 0.0, 20.0);
        });
    }
}
```
