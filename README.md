# Peach

[Processing](https://processing.org)-esque sandboxing library for Rust with [wgpu](https://github.com/gfx-rs/wgpu-rs).

## Example

See the [examples](examples/) folder for more code snippets, as well as (eventually) screen shots.

```rust
use peach::prelude::*;

fn main() -> SketchResult {
    let mut sketch = Sketch::builder("Example Sketch")
        .size(512.0, 512.0)
        .exit_key(Key::Escape)
        .build()?;

    sketch.run(draw);
}

fn draw(sketch: &mut Sketch) -> Graphics {
    let mut gfx = sketch.new_graphics();
    // Maybe Graphics::new(&sketch)??

    gfx.fill(0xFFFF4488);
    gfx.stroke(consts::BLACK); // same as 0xFF000000.

    gfx.stroke_weight(2.0);
    gfx.square(sketch.mouse(), 20.0);

    let t = sketch.secs_since_start();
    let x = 1.5 * t.cos();
    let y = (2.0 * t).sin();

    gfx.scoped(|gfx| {
        let center = sketch.center();
        let pos = center + Vector::new(x, y) * 100.0;

        gfx.stroke(consts::BLUE);
        gfx.align(Align::Center);
        gfx.rotate(x * PI);
        gfx.translate(pos);
        gfx.square(Vector::zero(), 10.0 + 20.0 * x.abs());
    });

    // Note that this will also have a black border, because the stroke set
    // inside of `scoped` is scoped to that closure.
    let bottom_right = sketch.corner_br();
    gfx.square(bottom_right - sketch.mouse(), 20.0);

    gfx
}
```

## Optional Model in Draw Method

```rust
use peach::prelude::*;

fn main() -> SketchResult {
    let mut sketch = Sketch::builder("Model example")
        .build()?;

    sketch.run(draw);
}

struct Model {
    x: f64,
}

impl FromSketch for Model {
    fn from_sketch(sketch: &Sketch) -> Self {
        Self {
            x: sketch.size().width / 2.0,
        }
    }
}

fn draw(sketch: &mut Sketch, model: &mut Model) -> Graphics {
    // ...
    // use and/or modify model within the sketch ...
    // ...
}
```
