use super::{construct::RawBuffersBuilder, context::Context};

use crate::tess::{self, path::iterator::FromPolyline};
use crate::types::{Index, Point, RawVertex, Size, Vector};

use anyhow::Result;

#[derive(Clone)]
enum DrawCommand {
    Draw(bool, Vec<Point>),
    UpdateContext(Context),
}

pub struct Graphics {
    draw_commands: Vec<DrawCommand>,
    context_stack: Vec<Context>,
    context_dirty: bool,
}

impl Graphics {
    pub(crate) fn new() -> Self {
        Self {
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

    fn update_context_if_dirty(&mut self) {
        if self.context_dirty {
            let context = self.context().clone();
            let command = DrawCommand::UpdateContext(context);

            self.draw_commands.push(command);
            self.context_dirty = false;
        }
    }

    fn draw(&mut self, points: &[Point], closed: bool) {
        self.update_context_if_dirty();
        let command = DrawCommand::Draw(closed, points.to_vec());
        self.draw_commands.push(command);
    }

    pub fn save(&mut self) {
        self.context_stack.push(self.context().clone());
    }

    pub fn restore(&mut self) {
        if self.context_stack.len() > 1 {
            self.context_stack.pop();
        }
        /*
        else {
            info!("Cannot restore context, as no context has been saved!");
        }
        */
    }

    pub fn rect(&mut self, position: Point, size: Size) {
        self.draw(
            &[
                position + Vector::zero(),
                position + Vector::new(size.width, 0.0),
                position + Vector::new(size.width, size.height),
            ],
            true,
        );
    }

    pub fn square(&mut self, position: Point, size: f32) {
        self.rect(position, Size::new(size, size));
    }

    pub(crate) fn construct_buffer_data(
        self,
    ) -> Result<(Vec<RawVertex>, Vec<Index>), tess::TessellationError> {
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
                }
                DrawCommand::UpdateContext(new_context) => {
                    builder.set_context(*new_context);
                    current_context = new_context;
                }
            }
        }

        Ok(builder.take())
    }
}
