use peach::prelude::*;

fn main() {
    let settings = Settings {
        title: Some("Example Sketch"),
        size: [512.0, 512.0].into(),
        framerate: None,
        ..Default::default()
    };

    peach::run::<Example>(settings).unwrap();
}

#[derive(Default)]
struct Example {
    x: f32,
}

impl Handler for Example {
    fn setup(sketch: &mut Sketch) -> Self {
        sketch.set_clear_color(Color::new(1.0, 1.0, 1.0, 1.0));
        Self::default()
    }

    fn update(&mut self, sketch: &mut Sketch, delta: Delta) {
        self.x = delta.since_start.as_secs_f32().sin();
    }

    fn draw(&self, sketch: &mut Sketch, gfx: &mut Graphics) {
        gfx.fill(Color {
            r: 1.0,
            g: 0.33,
            b: 0.66,
            a: 1.0,
        });
        gfx.no_stroke();

        gfx.rotate(Angle::radians(self.x * PI));
        gfx.square((self.x, 0.0), 0.25);
    }
}
