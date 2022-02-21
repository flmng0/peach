use crate::tess;
use crate::types::{Color, Transform};

#[derive(Debug, Clone, Copy)]
pub enum AnchorMode {
    First,
    Center,
}

#[derive(Debug, Clone, Copy)]
pub enum AngleMode {
    Radians,
    Degrees,
}

#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub angle_mode: AngleMode,
    pub anchor_mode: AnchorMode,
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
            angle_mode: AngleMode::Radians,
            anchor_mode: AnchorMode::Center,
            transform: Transform::IDENTITY,
            fill: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
            stroke: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
            stroke_weight: 1.0,
        }
    }
}
