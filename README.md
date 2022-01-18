# Peach

[Processing](https://processing.org)-esque sandboxing library for Rust with [wgpu](https://github.com/gfx-rs/wgpu-rs).

## Example

See the [examples](examples/) folder for more code snippets, as well as (eventually) screen shots.

```rust
use peach::prelude::*;

type Model = ();

#[peach::main]
fn main(sketch: &mut Sketch) (Drawer, ()) {
    sketch.set_title("Example Sketch");
    sketch.set_size(512.0, 512.0);
    sketch.set_exit_key(Some(Key::Escape));

    return (draw, ())
}

fn draw(sketch: &mut Sketch, model: &mut Model) -> Graphics {
    let gfx = sketch.new_graphics();
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
