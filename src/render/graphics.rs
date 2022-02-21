use anyhow::Result;

use super::construct::RawBuffersBuilder;
use super::context::{AnchorMode, AngleMode, Context};
use crate::consts::PI;
use crate::tess;
use crate::tess::path::iterator::FromPolyline;
use crate::types::*;

#[derive(Clone)]
enum DrawCommand {
    Draw(bool, Vec<Vector>),
    UpdateContext(Context),
}

pub(crate) struct BufferData {
    pub vertices: Vec<RawVertex>,
    pub indices: Vec<Index>,
}

#[derive(Clone)]
pub struct Graphics {
    pub(super) clear_color: Option<Color>,
    draw_commands: Vec<DrawCommand>,
    context: Context,
    context_dirty: bool,
}

impl Graphics {
    pub(crate) fn new(clear_color: Option<Color>) -> Self {
        Self {
            clear_color,
            draw_commands: Vec::new(),
            context: Context::default(),
            context_dirty: false,
        }
    }

    fn context(&self) -> &Context {
        &self.context
    }

    fn context_mut(&mut self) -> &mut Context {
        // Assume the context will be change if it has been
        // requested mutably.
        self.context_dirty = true;
        &mut self.context
    }

    fn transform(&self) -> &Transform {
        &self.context().transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.context_mut().transform
    }

    fn update_context(&mut self) {
        let context = self.context().clone();
        let command = DrawCommand::UpdateContext(context);

        self.draw_commands.push(command);
    }

    fn update_context_if_dirty(&mut self) {
        if self.context_dirty {
            self.update_context();
            self.context_dirty = false;
        }
    }

    fn align_points(&self, points: &[Vector], bounds: Option<BoundingBox>) -> Vec<Vector> {
        match self.context().anchor_mode {
            AnchorMode::First => points.to_vec(),
            AnchorMode::Center => {
                let bounds = bounds.unwrap_or_else(move || BoundingBox::from_points(points));

                let center = bounds.center();
                let diff = center - bounds.min;

                points.iter().map(|point| *point - diff).collect()
            },
        }
    }

    fn draw(&mut self, points: &[Vector], closed: bool, bounds: Option<BoundingBox>) {
        self.update_context_if_dirty();

        let points = self.align_points(points, bounds);

        let command = DrawCommand::Draw(closed, points);
        self.draw_commands.push(command);
    }

    fn radians_of(&self, angle: Scalar) -> Scalar {
        match self.context().angle_mode {
            AngleMode::Degrees => angle * (PI / 180.0),
            AngleMode::Radians => angle,
        }
    }

    pub fn scoped<C>(&mut self, mut cb: C)
    where
        C: FnMut(&mut Self),
    {
        let mut clone = self.clone();

        cb(&mut clone);

        self.draw_commands.extend_from_slice(&clone.draw_commands);

        self.update_context();
    }

    pub fn fill<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.context_mut().fill = Some(color.into());
    }

    pub fn no_fill(&mut self) {
        self.context_mut().fill = None;
    }

    pub fn stroke_weight(&mut self, weight: f32) {
        self.context_mut().stroke_weight = weight;
    }

    pub fn stroke<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.context_mut().stroke = Some(color.into());
    }

    pub fn no_stroke(&mut self) {
        self.context_mut().stroke = None;
    }

    pub fn anchor_mode(&mut self, mode: AnchorMode) {
        self.context_mut().anchor_mode = mode;
    }

    pub fn angle_mode(&mut self, mode: AngleMode) {
        self.context_mut().angle_mode = mode;
    }

    pub fn rotate(&mut self, angle: Scalar) {
        *self.transform_mut() = Transform::from_angle(self.radians_of(angle)) * *self.transform();
    }

    pub fn translate<V>(&mut self, by: V)
    where
        V: Into<Vector>,
    {
        *self.transform_mut() = Transform::from_translation(by.into()) * *self.transform();
    }

    pub fn rect<VA, VB>(&mut self, position: VA, size: VB)
    where
        VA: Into<Vector>,
        VB: Into<Vector>,
    {
        let position = position.into();
        let size = size.into();
        self.draw(
            &[
                position + Vector::ZERO,
                position + Vector::new(size.x, 0.0),
                position + Vector::new(size.x, size.y),
                position + Vector::new(0.0, size.y),
            ],
            true,
            Some(BoundingBox {
                min: position,
                max: position + size,
            }),
        );
    }

    pub fn square<V>(&mut self, position: V, size: Scalar)
    where
        V: Into<Vector>,
    {
        self.rect(position, Vector::new(size, size));
    }

    pub(crate) fn construct_buffer_data(self) -> Result<BufferData, tess::TessellationError> {
        let mut current_context = self.context();

        let mut builder = RawBuffersBuilder::default();

        let mut fill_tess = tess::FillTessellator::new();
        let mut stroke_tess = tess::StrokeTessellator::new();

        for command in self.draw_commands.iter() {
            match command {
                DrawCommand::Draw(closed, points) => {
                    let points = points.iter().map(|p| tess::math::point(p.x as _, p.y as _));

                    if current_context.fill.is_some() {
                        fill_tess.tessellate(
                            FromPolyline::new(*closed, points.clone()),
                            &current_context.get_fill_options(),
                            &mut builder,
                        )?;
                    }

                    if current_context.stroke.is_some() {
                        stroke_tess.tessellate(
                            FromPolyline::new(*closed, points),
                            &current_context.get_stroke_options(),
                            &mut builder,
                        )?;
                    }
                },
                DrawCommand::UpdateContext(new_context) => {
                    builder.set_context(*new_context);
                    current_context = new_context;
                },
            }
        }

        let (vertices, indices) = builder.take();
        Ok(BufferData { vertices, indices })
    }
}
