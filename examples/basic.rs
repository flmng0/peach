use peach::tess;

fn main() {
    let example = BasicExample {
        rect: Rectangle {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
        },
    };

    let sketch = peach::Sketch::new(Default::default()).unwrap();
    sketch.run(example);
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl peach::Drawable for Rectangle {
    fn draw(&self, vertex_buffers: &mut peach::VertexBuffers) {
        let rect = tess::math::Rect {
            origin: tess::math::Point::new(self.x, self.y),
            size: tess::math::Size::new(self.w, self.h),
        };

        tess::basic_shapes::fill_rectangle(
            &rect,
            &tess::FillOptions::tolerance(0.05),
            &mut tess::BuffersBuilder::new(
                vertex_buffers,
                peach::PathColor([1.0, 0.33, 0.66, 1.0].into()),
            ),
        )
        .unwrap();
    }
}

struct BasicExample {
    rect: Rectangle,
}

impl peach::Handler for BasicExample {
    fn update(&mut self, state: &peach::State) {
        self.rect.x = state.cursor.x as f32 - self.rect.w / 2.0;
        self.rect.y = state.cursor.y as f32 - self.rect.h / 2.0;
    }

    fn draw(&mut self, _state: &peach::State, renderer: &mut peach::Renderer) {
        renderer.draw(self.rect);
    }
}
