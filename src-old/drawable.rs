use crate::{
    tess::{self, FillVertex, VertexConstructor},
    Color, Point,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Point,
    pub color: Color,
}

impl Vertex {
    pub(crate) const SIZE: usize = std::mem::size_of::<Vertex>();
}

/// Type alias for [`lyon_tessellation::VertexBuffers`][0].
///
/// [0]: https://docs.rs/lyon_tessellation/0.14.2/lyon_tessellation/struct.VertexBuffers.html
pub type VertexBuffers = tess::VertexBuffers<Vertex, u32>;

pub trait Drawable {
    fn draw(&self, vertex_buffers: &mut VertexBuffers);
}

pub struct PathColor(pub Color);

impl VertexConstructor<FillVertex, Vertex> for PathColor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: Point::new(vertex.position.x, vertex.position.y),
            color: self.0,
        }
    }
}
