// Re-exports
pub use winit::event::{ModifiersState as Modifiers, MouseButton, VirtualKeyCode as Key};
pub use winit::window::Fullscreen;

// Structures
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct RawVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

unsafe impl bytemuck::Pod for RawVertex {}
unsafe impl bytemuck::Zeroable for RawVertex {}

// Common types
pub type Index = u32;
pub type Color = rgb::RGBA<f32>;

macro_rules! define_euclid {
    ($public_type:ident, $euclid_type:ident) => {
        pub type $public_type = euclid::default::$euclid_type<f32>;
    };
}

define_euclid!(Point, Point2D);
define_euclid!(Rotation, Rotation2D);
define_euclid!(Size, Size2D);
define_euclid!(Transform, Transform2D);
define_euclid!(Translation, Translation2D);
define_euclid!(Vector, Vector2D);
define_euclid!(BoundingBox, Box2D);

pub type Angle = euclid::Angle<f32>;
