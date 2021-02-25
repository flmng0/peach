use crate::{
    tess,
    types::{Color, Transform},
};

#[derive(Debug, Copy, Clone)]
pub struct Context {
    pub transform: Transform,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_weight: f32,
}

impl Context {
    pub fn get_fill_options(&self) -> tess::FillOptions {
        tess::FillOptions::default() //.with_fill_rule(self.fill_rule.into())
    }

    pub fn get_stroke_options(&self) -> tess::StrokeOptions {
        // get_stroke_options should only be used when there is a
        // stroke defined.
        let width = self.stroke_weight;

        tess::StrokeOptions::default().with_line_width(width)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            transform: Transform::identity(),
            fill: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
            stroke: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
            stroke_weight: 1.0,
        }
    }
}
