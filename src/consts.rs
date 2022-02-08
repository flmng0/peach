pub use std::f64::consts::*;

pub mod colors {
    use crate::constructors::rgba;
    use crate::types::Color;

    pub const BLACK: Color = rgba(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Color = rgba(1.0, 1.0, 1.0, 1.0);
    pub const RED: Color = rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = rgba(0.0, 0.0, 1.0, 1.0);
}
