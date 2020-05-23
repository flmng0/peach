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
    fn draw(&self, gfx: &mut Graphics) {
        let center = sketch.center();

        gfx.scope(|gfx| {
            gfx.anchorMode(AnchorMode::Center);
            gfx.translate(center.x + self.x * 200.0, center.y);
            gfx.rotate(Angle::Radians(self.x * PI));
            gfx.square(0.0, 0.0, 20.0);
        });
    }

    fn update(&mut self, sketch: &mut Sketch, delta: Delta) {
        self.x = delta.time_since_start.as_secs_f32().sin();
    }
}
```
