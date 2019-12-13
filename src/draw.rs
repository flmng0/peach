use crate::{
    color::Color,
    vertex::{VertexBuffer, WithColorAndTransform},
    Point, Rect, Size, Transform, Vector,
};

use crate::tess::{basic_shapes as ba, BuffersBuilder, FillOptions, LineCap, StrokeOptions};

#[derive(Debug, Copy, Clone)]
/// Used in [`Sketch::anchor`][0]; describes the anchor
/// point where all geometries are drawn from.
///
/// Default Value: `Anchor::TopLeft`.
///
/// # Applies To:
/// - `rect`
///
/// [0]: struct.Sketch.html#method.anchor
pub enum Anchor {
    /// Place the top-left corner of geometries under the
    /// given position.
    TopLeft,
    /// Place geometries in the center of the given
    /// position.
    Center,
    /// Offset geometries by the given pixels.
    Offset(f32, f32),
    /// Offset by the given percentage.
    Percent(f32, f32),
}

impl Default for Anchor {
    fn default() -> Anchor {
        Anchor::TopLeft
    }
}

#[derive(Debug, Copy, Clone)]
// TODO: Document
pub enum AngleMode {
    Degrees,
    Radians,
}

impl Default for AngleMode {
    fn default() -> AngleMode {
        AngleMode::Degrees
    }
}

#[derive(Debug, Copy, Clone)]
/// Drawing state stored by the [`Drawing`][0] trait
/// implementor.
///
/// [0]: trait.Drawing.html
pub struct DrawState {
    fill_color: Option<Color>,
    stroke_color: Option<Color>,

    fill_options: FillOptions,
    stroke_options: StrokeOptions,

    anchor: Anchor,
    angle_mode: AngleMode,

    transform: Transform,
}

impl Default for DrawState {
    fn default() -> DrawState {
        let fill_options = FillOptions::default().with_normals(false);
        let stroke_options = StrokeOptions::default().with_line_cap(LineCap::Round);

        DrawState {
            fill_color: Some(Color::WHITE),
            stroke_color: Some(Color::BLACK),
            fill_options,
            stroke_options,
            anchor: Anchor::default(),
            angle_mode: AngleMode::default(),
            transform: Transform::identity(),
        }
    }
}

pub trait Drawing {
    // TODO: Document
    fn draw_state(&mut self) -> &mut DrawState;
    // TODO: Document
    fn fill_buffer(&mut self) -> &mut VertexBuffer;
    // TODO: Document
    fn stroke_buffer(&mut self) -> &mut VertexBuffer;

    // TODO: Document
    fn fill(&mut self, color: Color) {
        self.draw_state().fill_color = Some(color);
    }

    // TODO: Document
    fn no_fill(&mut self) {
        self.draw_state().fill_color = None;
    }

    // TODO: Document
    fn stroke(&mut self, color: Color) {
        self.draw_state().stroke_color = Some(color);
    }

    // TODO: Document
    fn no_stroke(&mut self) {
        self.draw_state().stroke_color = None;
    }

    // TODO: Document
    fn stroke_weight(&mut self, weight: f32) {
        self.draw_state().stroke_options.line_width = weight;
    }

    // TODO: Document
    fn anchor(&mut self, anchor: Anchor) {
        self.draw_state().anchor = anchor;
    }

    fn angle_mode(&mut self, angle_mode: AngleMode) {
        self.draw_state().angle_mode = angle_mode;
    }

    fn translate(&mut self, x: f32, y: f32) {
        let draw_state = self.draw_state();

        let mut translation = Transform::identity();
        translation.m41 = x;
        translation.m42 = y;

        // TODO translation.m43 = z
        //
        // Although, when 3D is implemented, euclids built-in
        // translate method can be used.

        draw_state.transform = draw_state.transform.post_transform(&translation);
    }

    // TODO: Document
    fn rotate(&mut self, angle: f32) {
        let draw_state = self.draw_state();

        let angle = match draw_state.angle_mode {
            AngleMode::Degrees => euclid::Angle::degrees(angle),
            AngleMode::Radians => euclid::Angle::radians(angle),
        };

        draw_state.transform = draw_state.transform.post_rotate(0.0, 0.0, 1.0, angle);
    }

    // TODO: Document
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let draw_state = *self.draw_state();

        let point = Point::new(x, y);
        let size = Size::new(w, h);

        let pos = apply_anchor(draw_state.anchor, point, size);
        let rect = Rect::new(pos, size);

        if let Some(fill_color) = draw_state.fill_color {
            ba::fill_rectangle(
                &rect,
                &draw_state.fill_options,
                &mut BuffersBuilder::new(
                    self.fill_buffer(),
                    WithColorAndTransform(fill_color, draw_state.transform),
                ),
            )
            .unwrap();
        }

        if let Some(stroke_color) = draw_state.stroke_color {
            ba::stroke_rectangle(
                &rect,
                &draw_state.stroke_options,
                &mut BuffersBuilder::new(
                    self.stroke_buffer(),
                    WithColorAndTransform(stroke_color, draw_state.transform),
                ),
            )
            .unwrap();
        }
    }
}

fn apply_anchor(anchor: Anchor, point: Point, size: Size) -> Point {
    match anchor {
        Anchor::TopLeft => point,
        Anchor::Center => point - size.to_vector() / 2.0,
        Anchor::Offset(x, y) => point - Vector::new(x, y),
        Anchor::Percent(x, y) => point - Vector::new(x * size.width, y * size.height),
    }
}
