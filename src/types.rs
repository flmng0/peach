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
pub type Scalar = f64;

pub type GpuScalar = f32;
pub type Index = u32;

pub type Color = rgb::RGBA<Scalar>;

pub type Vector = glam::DVec2;
pub type Transform = glam::DAffine2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Vector,
    pub max: Vector,
}

impl BoundingBox {
    pub const fn new(min: Vector, max: Vector) -> Self {
        Self { min, max }
    }

    pub fn from_components(min_x: Scalar, min_y: Scalar, max_x: Scalar, max_y: Scalar) -> Self {
        Self {
            min: Vector::new(min_x, min_y),
            max: Vector::new(max_x, max_y),
        }
    }

    pub fn from_points(points: &[Vector]) -> Self {
        let mut min_x = Scalar::INFINITY;
        let mut min_y = Scalar::NEG_INFINITY;

        let mut max_x = Scalar::INFINITY;
        let mut max_y = Scalar::NEG_INFINITY;

        for point in points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);

            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        Self::from_components(min_x, min_y, max_x, max_y)
    }

    pub fn center(&self) -> Vector {
        (self.min + self.max) / 2.0
    }
}

// Consider when implementing 3D drawing.
// pub type Vector3D = glam::Vec4;
// pub type Matrix3D = glam::Mat4;
