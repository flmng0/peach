# Peach
[Processing](https://processing.org)-esque sandboxing library for Rust with [wgpu](https://github.com/gfx-rs/wgpu-rs).

# Example
See the [examples](examples/) folder for more code snippets, as well as screen shots.
```rust
use peach::prelude::*;

fn main() {
    peach::run::<Example>(Settings {
        title: Some("Example Sketch"),
        size: [512.0, 512.0],
        ..Default::default()
    });
}

#[derive(Default)]
struct Example {
    x: f32,
}

impl Handler for Example {
    fn setup(sketch: &mut Sketch) -> Self {
        sketch.fill(Color::from_hex(0xff4488ff));
        sketch.no_stroke();

        Self::default()
    }

    fn draw(&self, sketch: &mut Sketch, gfx: &mut Graphics) {
        let center = sketch.center();

        gfx.save();
            gfx.anchorMode(AnchorMode::Center);
            gfx.translate(center);
            gfx.rotate(Angle::Radians(self.x * PI));
            gfx.square((self.x * 200.0, 0.0), 20.0);
        gfx.restore();
    }

    fn update(&mut self, sketch: &mut Sketch, delta: Delta) {
        self.x = delta.time_since_start.as_secs_f32().sin();
    }
}
```
