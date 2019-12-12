use crate::{color::Color, Point};
use tess::{FillVertex, StrokeVertex, VertexBuffers, VertexConstructor};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Point,
    pub color: Color,
    pub index: u32,
}

pub struct WithIndexAndColor(pub u32, pub Color);

pub type VertexBuffer = VertexBuffers<Vertex, u32>;

impl VertexConstructor<FillVertex, Vertex> for WithIndexAndColor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let WithIndexAndColor(index, color) = *self;

        Vertex {
            position: vertex.position,
            color,
            index,
        }
    }
}

impl VertexConstructor<StrokeVertex, Vertex> for WithIndexAndColor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        let WithIndexAndColor(index, color) = *self;

        Vertex {
            position: vertex.position,
            color,
            index,
        }
    }
}
