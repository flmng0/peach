use crate::tess::{self, FillVertex, VertexConstructor};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub(crate) const SIZE: usize = std::mem::size_of::<Vertex>();
}

pub type VertexBuffers = tess::VertexBuffers<Vertex, u32>;

pub trait Drawable {
    fn draw(&self, vertex_buffers: &mut VertexBuffers);
}

pub struct PathColor(pub [f32; 4]);

impl VertexConstructor<FillVertex, Vertex> for PathColor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position.x, vertex.position.y],
            color: self.0,
        }
    }
}
