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
}
