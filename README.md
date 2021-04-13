# Peach
[Processing](https://processing.org)-esque sandboxing library for Rust with [wgpu](https://github.com/gfx-rs/wgpu-rs).

# Example
See the [examples](examples/) folder for more code snippets, as well as screen shots.
```rust
use peach::prelude::*;

fn main() {
    let settings = Settings {
        title: Some("Example Sketch"),
        size: [512.0, 512.0].into(),
        framerate: None,
        exit_key: Some(Key::Escape),
        ..Default::default()
    };

    peach::run::<Example>(settings).unwrap();
}

#[derive(Default)]
struct Example {
    x: Scalar,
}

impl Handler for Example {
    fn setup(sketch: &mut Sketch) -> Self {
        sketch.set_clear_color(Color::new(1.0, 1.0, 1.0, 1.0));
        Self::default()
    }

    fn update(&mut self, _sketch: &mut Sketch, delta: Delta) {
        self.x = delta.since_start.as_secs_scalar().sin();
    }

    #[rustfmt::skip]
    fn draw(&self, sketch: &mut Sketch, gfx: &mut Graphics) {
        gfx.fill(Color {
            r: 1.0,
            g: 0.33,
            b: 0.66,
            a: 1.0,
        });

        let size = Point::from(sketch.get_size().to_tuple());
        let center = size / 2.0;
        let pos = center.to_vector() + Vector::new(self.x, 0.0) * 100.0;

        gfx.stroke(colors::BLACK);
        gfx.stroke_weight(2.0);
        gfx.square(sketch.get_mouse_position(), 20.0);

        gfx.save();
            gfx.stroke(colors::BLUE);
            gfx.anchor_mode(AnchorMode::Center);
            gfx.rotate(Angle::radians(self.x * PI));
            gfx.translate(pos);
            gfx.square(Point::zero(), 10.0 + 20.0 * self.x.sin().abs());
        gfx.restore();

        gfx.square( size - sketch.get_mouse_position().to_vector(), 20.0);
    }
}

```
