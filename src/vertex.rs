use crate::{color::Color, Point};
use tess::{FillVertex, StrokeVertex, VertexBuffers, VertexConstructor};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Point,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Point, color: Color) -> Vertex {
        Vertex { position, color }
    }
}

pub type VertexBuffer = VertexBuffers<Vertex, u32>;

impl VertexConstructor<FillVertex, Vertex> for Color {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: vertex.position,
            color: *self,
        }
    }
}

impl VertexConstructor<StrokeVertex, Vertex> for Color {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: vertex.position,
            color: *self,
        }
    }
}
