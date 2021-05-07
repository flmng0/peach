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
struct Example;

impl Handler for Example {
    fn setup(sketch: &mut Sketch) -> Self {
        sketch.set_clear_color(Color::new(1.0, 1.0, 1.0, 1.0));
        Self::default()
    }

    #[rustfmt::skip]
    fn draw(&mut self, sketch: &mut Sketch, gfx: &mut Graphics) {
        gfx.fill(Color {
            r: 1.0,
            g: 0.33,
            b: 0.66,
            a: 1.0,
        });

        let t = sketch.get_time_since_start().as_secs_scalar();
        let x = 1.5 * t.cos();
        let y = (2.0 * t).sin();

        let size = Point::from(sketch.get_size().to_tuple());
        let center = sketch.get_center();
        let pos = center.to_vector() + Vector::new(x, y) * 100.0;

        gfx.stroke(colors::BLACK);
        gfx.stroke_weight(2.0);
        gfx.square(sketch.get_mouse_position(), 20.0);

        gfx.save();
            gfx.stroke(colors::BLUE);
            gfx.anchor_mode(AnchorMode::Center);
            gfx.rotate(Angle::radians(x * PI));
            gfx.translate(pos);
            gfx.square(Point::zero(), 10.0 + 20.0 * x.abs());
        gfx.restore();

        gfx.fill(colors::BLUE);
        gfx.square( size - sketch.get_mouse_position().to_vector(), 20.0);
    }
}
