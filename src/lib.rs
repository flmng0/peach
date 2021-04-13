pub mod render;
pub mod sketch;
pub mod types;

pub use {lyon_tessellation as tess, rgb};

pub use self::sketch::run::run;

pub mod prelude {
    #[cfg(feature = "force_f32")]
    pub use std::f32::consts::*;
    #[cfg(not(feature = "force_f32"))]
    pub use std::f64::consts::*;

    pub use crate::render::*;
    pub use crate::sketch::*;
    pub use crate::types::*;

    #[rustfmt::skip]
    pub mod colors {
        use super::Color;

        pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        pub const RED  : Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
        pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
        pub const BLUE : Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    }
}
