use peach::prelude::*;

fn main() {
    // Settings for the window, see the inline documentation for
    // more information.
    let settings = Settings {
        title: Some("Example Sketch"),
        size: [512.0, 512.0].into(),
        framerate: None,
        exit_key: Some(Key::Escape),
        ..Default::default()
    };

    // Run a given sketch, provided a structure type
    // implementing `Handler`.
    peach::run::<Example>(settings).unwrap();
}

#[derive(Default)]
struct Example;

// Event handler, such as draw, mouse moved, mouse pressed,
// key released, etc.
impl Handler for Example {
    fn setup(sketch: &mut Sketch) -> Self {
        sketch.set_clear_color(Color::new(1.0, 1.0, 1.0, 1.0));
        Self::default()
    }

    fn draw(&mut self, sketch: &mut Sketch, gfx: &mut Graphics) {
        let t = sketch.get_time_since_start().as_secs_scalar();
        let x = 1.5 * t.cos();
        let y = (2.0 * t).sin();

        let size = Point::from(sketch.get_size().to_tuple());

        // Set the fill color for all 'top-level' objects.
        gfx.fill(Color {
            r: 1.0,
            g: 0.33,
            b: 0.66,
            a: 1.0,
        });

        // Draw an initial square at the cursor position.
        gfx.stroke(colors::BLACK);
        gfx.stroke_weight(2.0);
        gfx.square(sketch.get_mouse_position(), 20.0);

        // A `scoped` block, similar to a push-pop block in
        // Processing.
        //
        // Can be infinitely nested.
        gfx.scoped(|gfx| {
            let center = sketch.get_center();
            let pos = center.to_vector() + Vector::new(x, y) * 100.0;

            gfx.stroke(colors::BLUE);
            gfx.anchor_mode(AnchorMode::Center);
            gfx.rotate(Angle::radians(x * PI));
            gfx.translate(pos);
            gfx.square(Point::zero(), 10.0 + 20.0 * x.abs());
        });

        gfx.fill(colors::BLUE);

        // Stroke will still be black, because scoped blocks have no
        // effect outside of their scope.
        gfx.square(size - sketch.get_mouse_position().to_vector(), 20.0);
    }
}
