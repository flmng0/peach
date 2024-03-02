use peach::prelude::*;

fn main() {
    let sketch = Sketch::builder("Example Sketch")
        .size(512.0, 512.0)
        .exit_key(Key::Escape)
        .build();

    sketch.run(draw);
}

type Model = ();

fn draw(sketch: &mut Sketch, _model: &mut Model) -> Graphics {
    let mut gfx = sketch.new_graphics();
    // Maybe Graphics::new(&sketch)??

    gfx.fill(hex(0xFFFF4488));
    gfx.stroke(colors::BLACK); // same as 0xFF000000.

    gfx.stroke_weight(2.0);
    gfx.square(sketch.mouse(), 20.0);

    let t = sketch.secs_since_start();
    let x = 1.5 * t.cos();
    let y = (2.0 * t).sin();

    gfx.scoped(|gfx| {
        let center = sketch.center();
        let pos = center + Vector::new(x, y) * 100.0;

        gfx.stroke(colors::BLUE);
        gfx.anchor_mode(AnchorMode::Center);
        gfx.rotate(x * PI);
        gfx.translate(pos);
        gfx.square(Vector::ZERO, 10.0 + 20.0 * x.abs());
    });

    gfx.fill(hex(0xff282a36));

    // Note that this will also have a black border, because the
    // stroke set inside of `scoped` is scoped to that
    // closure.
    let bottom_right = sketch.corner_br();
    gfx.square(bottom_right - sketch.mouse(), 20.0);

    gfx
}
