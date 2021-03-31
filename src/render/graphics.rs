use anyhow::Result;

use super::construct::RawBuffersBuilder;
use super::context::{AnchorMode, Context};
use crate::tess;
use crate::tess::path::iterator::FromPolyline;
use crate::types::*;

#[derive(Clone)]
enum DrawCommand {
    Draw(bool, Vec<Point>),
    UpdateContext(Context),
}

pub(crate) struct BufferData {
    pub vertices: Vec<RawVertex>,
    pub indices: Vec<Index>,
}

pub struct Graphics {
    pub(super) clear_color: Option<Color>,
    draw_commands: Vec<DrawCommand>,
    context_stack: Vec<Context>,
    context_dirty: bool,
}

impl Graphics {
    pub(crate) fn new(clear_color: Option<Color>) -> Self {
        Self {
            clear_color,
            draw_commands: Vec::new(),
            context_stack: vec![Context::default()],
            context_dirty: false,
        }
    }

    fn context(&self) -> &Context {
        self.context_stack
            .last()
            .expect("Uh oh! Impossible thing happened!")
    }

    fn context_mut(&mut self) -> &mut Context {
        // Assume the context will be change if it has been
        // requested mutably.
        self.context_dirty = true;
        self.context_stack
            .last_mut()
            .expect("Uh oh! Impossible thing happened!")
    }

    fn transform(&self) -> &Transform {
        &self.context().transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.context_mut().transform
    }

    fn update_context_if_dirty(&mut self) {
        if self.context_dirty {
            let context = self.context().clone();
            let command = DrawCommand::UpdateContext(context);

            self.draw_commands.push(command);
            self.context_dirty = false;
        }
    }

    fn align_points(&self, points: &[Point], bounds: Option<BoundingBox>) -> Vec<Point> {
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

    fn draw(&mut self, points: &[Point], closed: bool, bounds: Option<BoundingBox>) {
        self.update_context_if_dirty();

        let points = self.align_points(points, bounds);

        let command = DrawCommand::Draw(closed, points);
        self.draw_commands.push(command);
    }

    pub fn save(&mut self) {
        self.context_stack.push(self.context().clone());
    }

    pub fn restore(&mut self) {
        if self.context_stack.len() > 1 {
            self.context_stack.pop();
        }
        // else {
        // info!("Cannot restore context, as no context has
        // been saved!"); }
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

    pub fn rotate(&mut self, angle: Angle) {
        *self.transform_mut() = self.transform().pre_rotate(angle);
    }

    pub fn translate<V>(&mut self, by: V)
    where
        V: Into<Vector>,
    {
        *self.transform_mut() = self.transform().pre_translate(by.into());
    }

    pub fn rect<P, S>(&mut self, position: P, size: S)
    where
        P: Into<Point>,
        S: Into<Size>,
    {
        let position = position.into();
        let size = size.into();
        self.draw(
            &[
                position + Vector::zero(),
                position + Vector::new(size.width, 0.0),
                position + Vector::new(size.width, size.height),
                position + Vector::new(0.0, size.height),
            ],
            true,
            Some(BoundingBox {
                min: position,
                max: position + size.to_vector(),
            }),
        );
    }

    pub fn square<P>(&mut self, position: P, size: f32)
    where
        P: Into<Point>,
    {
        self.rect(position, Size::new(size, size));
    }

    pub(crate) fn construct_buffer_data(self) -> Result<BufferData, tess::TessellationError> {
        let mut current_context = self.context();

        let mut builder = RawBuffersBuilder::default();

        let mut fill_tess = tess::FillTessellator::new();
        let mut stroke_tess = tess::StrokeTessellator::new();

        for command in self.draw_commands.iter() {
            match command {
                DrawCommand::Draw(closed, points) => {
                    if current_context.fill.is_some() {
                        fill_tess.tessellate(
                            FromPolyline::new(*closed, points.iter().cloned()),
                            &current_context.get_fill_options(),
                            &mut builder,
                        )?;
                    }

                    if current_context.stroke.is_some() {
                        stroke_tess.tessellate(
                            FromPolyline::new(*closed, points.iter().cloned()),
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
