use std::time::Duration;

// Re-exports
pub use winit::event::{ModifiersState as Modifiers, MouseButton, VirtualKeyCode as Key};
pub use winit::window::Fullscreen;

// Structures
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct RawVertex {
    pub position: [GpuScalar; 2],
    pub color: [GpuScalar; 4],
}

unsafe impl bytemuck::Pod for RawVertex {}
unsafe impl bytemuck::Zeroable for RawVertex {}

// Common types
#[cfg(feature = "force_f32")]
pub type Scalar = f32;
#[cfg(not(feature = "force_f32"))]
pub type Scalar = f64;

pub type GpuScalar = f32;
pub type Index = u32;
pub type Color = rgb::RGBA<Scalar>;

macro_rules! define_euclid {
    ($public_type:ident, $euclid_type:ident) => {
        pub type $public_type = euclid::default::$euclid_type<Scalar>;
    };
}

define_euclid!(Point, Point2D);
define_euclid!(Rotation, Rotation2D);
define_euclid!(Size, Size2D);
define_euclid!(Transform, Transform2D);
define_euclid!(Translation, Translation2D);
define_euclid!(Vector, Vector2D);
define_euclid!(BoundingBox, Box2D);

pub type Angle = euclid::Angle<Scalar>;

pub trait ScalarDuration {
    fn as_secs_scalar(&self) -> Scalar;
}

impl ScalarDuration for Duration {
    #[cfg(feature = "force_f32")]
    fn as_secs_scalar(&self) -> Scalar {
        self.as_secs_f32()
    }

    #[cfg(not(feature = "force_f32"))]
    fn as_secs_scalar(&self) -> Scalar {
        self.as_secs_f64()
    }
}
