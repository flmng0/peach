use crate::tess::{FillVertex, StrokeVertex, VertexBuffers, VertexConstructor};
use crate::{color::Color, Point, Transform};

// type Vector3 = euclid::default::Vector3D<f32>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Point,
    pub color: Color,
}

pub struct WithColorAndTransform(pub Color, pub Transform);

pub type VertexBuffer = VertexBuffers<Vertex, u32>;

impl VertexConstructor<FillVertex, Vertex> for WithColorAndTransform {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let WithColorAndTransform(color, transform) = *self;

        let point = vertex.position.to_3d();
        let position = transform.transform_point3d(point).unwrap().to_2d();

        Vertex { position, color }
    }
}

impl VertexConstructor<StrokeVertex, Vertex> for WithColorAndTransform {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        let WithColorAndTransform(color, transform) = *self;

        let point = vertex.position.to_3d();
        dbg!(point);
        let position = transform.transform_point3d(point).unwrap().to_2d();
        dbg!(position);

        Vertex { position, color }
    }
}
