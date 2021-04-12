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

    fn draw(&self, sketch: &mut Sketch, gfx: &mut Graphics) {
        gfx.fill(Color {
            r: 1.0,
            g: 0.33,
            b: 0.66,
            a: 1.0,
        });
        gfx.no_stroke();

        let center = sketch.get_size() / 2.0;
        let pos = center.to_vector() + Vector::new(self.x, 0.0) * 100.0;

        gfx.anchor_mode(AnchorMode::Center);
        gfx.translate(pos);
        gfx.rotate(Angle::radians(self.x * PI));
        gfx.square(Point::zero(), 10.0 + 20.0 * self.x.sin().abs());
    }
}
