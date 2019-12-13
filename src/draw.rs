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
/// Describes what type of angle to use for rotation
/// operations.
///
/// Default value: `Degrees`.
pub enum AngleMode {
    /// Use degrees.
    Degrees,
    /// Use radians.
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
    /// Current fill color.
    fill_color: Option<Color>,
    /// Current stroke color.
    stroke_color: Option<Color>,
    /// Fill options cached for sharing between states.
    fill_options: FillOptions,
    /// Stroke options cached for sharing between states.
    stroke_options: StrokeOptions,
    /// Describes where to draw shapes based on different
    /// factors.
    anchor: Anchor,
    /// Describes which type of angle to use.
    angle_mode: AngleMode,
    /// Current transformation matrix, used for scale,
    /// rotation, etc.
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

/// Different methods for drawing basic shapes and setting
/// the drawing state.
pub trait Drawing {
    /// Unless implementing a new backend beside OpenGL,
    /// don't worry about this, `fill_buffer`,
    /// `stroke_buffer`, and `size`.
    fn draw_state(&mut self) -> &mut DrawState;
    fn fill_buffer(&mut self) -> &mut VertexBuffer;
    fn stroke_buffer(&mut self) -> &mut VertexBuffer;
    fn size(&self) -> Size;

    /// Clear the window completely, including the
    /// background.
    fn clear(&mut self) {
        let fill_buffer = self.fill_buffer();
        fill_buffer.vertices.clear();
        fill_buffer.indices.clear();

        let stroke_buffer = self.stroke_buffer();
        stroke_buffer.vertices.clear();
        stroke_buffer.indices.clear();
    }

    /// Draw a background to the window with the given
    /// `color`.
    fn background(&mut self, color: Color) {
        let draw_state = *self.draw_state();

        let rect = Rect::new(Point::zero(), self.size());

        ba::fill_rectangle(
            &rect,
            &draw_state.fill_options,
            &mut BuffersBuilder::new(
                self.fill_buffer(),
                WithColorAndTransform(color, Transform::identity()),
            ),
        )
        .unwrap();
    }

    /// Set the fill color of the current state to the given
    /// `color`.
    fn fill(&mut self, color: Color) {
        self.draw_state().fill_color = Some(color);
    }

    /// Disable filling.
    fn no_fill(&mut self) {
        self.draw_state().fill_color = None;
    }

    // Set the stroke color of the current state to the given
    // `color`.
    fn stroke(&mut self, color: Color) {
        self.draw_state().stroke_color = Some(color);
    }

    /// Disable drawing stroke.
    fn no_stroke(&mut self) {
        self.draw_state().stroke_color = None;
    }

    /// Set the thickness of the stroke to the given `width`
    /// in pixels.
    fn stroke_weight(&mut self, weight: f32) {
        self.draw_state().stroke_options.line_width = weight;
    }

    /// Set the anchor mode to `anchor`.
    fn anchor(&mut self, anchor: Anchor) {
        self.draw_state().anchor = anchor;
    }

    /// Set the angle mode to `angle_mode`.
    fn angle_mode(&mut self, angle_mode: AngleMode) {
        self.draw_state().angle_mode = angle_mode;
    }

    /// Translate every proceeding draw operation by (`x`,
    /// `y`) pixels.
    ///
    /// This can be stacked, e.g:
    /// ```no_run
    /// sketch.translate(20.0, 20.0);
    /// sketch.translate(40.0, 30.0);
    /// ```
    /// The total translation in the snippet above is
    /// (`60.0`, `50.0`).
    fn translate(&mut self, x: f32, y: f32) {
        let draw_state = self.draw_state();

        let mut translation = Transform::identity();
        translation.m41 = x;
        translation.m42 = y;

        draw_state.transform = draw_state.transform.pre_transform(&translation);
    }

    /// Rotate by the given `angle` counter-clockwise.
    /// Whether the angle is represented by radians or
    /// degrees depends on the current angle mode, which can
    /// be set by [`angle_mode`][0].
    ///
    /// [0]: trait.Drawing.html#method.angle_mode
    fn rotate(&mut self, angle: f32) {
        let draw_state = self.draw_state();

        let angle = match draw_state.angle_mode {
            AngleMode::Degrees => euclid::Angle::degrees(angle),
            AngleMode::Radians => euclid::Angle::radians(angle),
        };

        draw_state.transform = draw_state.transform.pre_rotate(0.0, 0.0, 1.0, angle);
    }

    /// Draw a rectangle at the given position (`x`, `y`),
    /// with size (`w`, `h`).
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

    /// Draw a square at the given position (`x`, `y`), with
    /// side length `l`.
    fn square(&mut self, x: f32, y: f32, l: f32) {
        self.rect(x, y, l, l);
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
