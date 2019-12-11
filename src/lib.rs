pub mod color;
pub mod config;
pub mod run;
pub mod sketch;

pub use euclid;
pub use lyon_tessellation as tess;
pub use winit::event::{ModifiersState, MouseButton, VirtualKeyCode};

macro_rules! euclid_type {
    ($type:ident) => {
        pub type $type = euclid::default::$type<f32>;
    };
    ($output:ident, $input:ident) => {
        pub type $output = euclid::default::$input<f32>;
    };
}

euclid_type!(Rect);

euclid_type!(Point, Point2D);
euclid_type!(Size, Size2D);
euclid_type!(Transform, Transform3D);
