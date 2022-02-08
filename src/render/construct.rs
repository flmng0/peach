//! Tessallation constructors

use super::context::Context;
use crate::tess;
use crate::types::{Color, GpuScalar, Index, Point, RawVertex};

type GeometryBuilderResult = Result<tess::VertexId, tess::GeometryBuilderError>;

#[derive(Default)]
pub(super) struct RawBuffersBuilder {
    vertices: Vec<RawVertex>,
    indices: Vec<Index>,

    offsets: (Index, Index),

    context: Context,
}

impl RawBuffersBuilder {
    fn add_vertex(&mut self, position: Point, color: Color) -> GeometryBuilderResult {
        use rgb::ComponentMap;

        let position: [GpuScalar; 2] = self.context.transform.transform_point(position).cast().into();
        let color: [GpuScalar; 4] = color.map(|p| p as GpuScalar).into();

        self.vertices.push(RawVertex { position, color });

        let len = self.vertices.len();
        if len > Index::MAX as usize {
            return Err(tess::GeometryBuilderError::TooManyVertices);
        }

        let id = (len as Index - 1) - self.offsets.0;
        Ok(tess::VertexId(id))
    }

    pub fn set_context(&mut self, context: Context) {
        self.context = context;
    }

    pub fn take(self) -> (Vec<RawVertex>, Vec<Index>) {
        (self.vertices, self.indices)
    }
}

impl tess::GeometryBuilder for RawBuffersBuilder {
    // Re-set offsets to the length of each buffer when building
    // the geometry begins.
    fn begin_geometry(&mut self) {
        self.offsets = (self.vertices.len() as Index, self.indices.len() as Index);
    }

    // When the geometry is finished, tell lyon how many
    // vertices and indices were added.
    fn end_geometry(&mut self) -> tess::Count {
        tess::Count {
            vertices: self.vertices.len() as Index - self.offsets.0,
            indices: self.indices.len() as Index - self.offsets.1,
        }
    }

    // Add the indices for each vertex to the indices list.
    fn add_triangle(&mut self, a: tess::VertexId, b: tess::VertexId, c: tess::VertexId) {
        let offset = self.offsets.0;
        self.indices.push(Index::from(a) + offset);
        self.indices.push(Index::from(b) + offset);
        self.indices.push(Index::from(c) + offset);
    }

    // Remove all the added vertices/indices from the list
    // on abort.
    fn abort_geometry(&mut self) {
        self.vertices.truncate(self.offsets.0 as usize);
        self.indices.truncate(self.offsets.1 as usize);
    }
}

impl tess::FillGeometryBuilder for RawBuffersBuilder {
    fn add_fill_vertex(&mut self, vertex: tess::FillVertex) -> GeometryBuilderResult {
        let position = vertex.position();
        if let Some(color) = self.context.fill {
            self.add_vertex(position.cast(), color)
        }
        else {
            Err(tess::GeometryBuilderError::InvalidVertex)
        }
    }
}

impl tess::StrokeGeometryBuilder for RawBuffersBuilder {
    fn add_stroke_vertex(&mut self, vertex: tess::StrokeVertex) -> GeometryBuilderResult {
        let position = vertex.position();
        if let Some(color) = self.context.stroke {
            self.add_vertex(position.cast(), color)
        }
        else {
            Err(tess::GeometryBuilderError::InvalidVertex)
        }
    }
}
