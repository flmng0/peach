pub mod color;
pub mod config;
pub mod run;
pub mod sketch;
pub mod state;

pub use euclid;
pub use winit::event::{ModifiersState as Modifiers, MouseButton as Button, VirtualKeyCode as Key};

pub(crate) use lyon_tessellation as tess;
pub(crate) mod vertex;

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
euclid_type!(Vector, Vector2D);

pub use crate::run::run;
pub mod prelude {
    pub use crate::{
        color::Color,
        config::{Callbacks, Config},
        sketch::{Anchor, Sketch},
        state::State,
        Button, Key, Modifiers, Point, Rect, Size, Transform, Vector,
    };
}
