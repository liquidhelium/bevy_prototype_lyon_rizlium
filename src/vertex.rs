use bevy::{render::color::Color, prelude::Vec2};
use lyon_tessellation::{
    self as tess, FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};

use crate::brush::{Brush, Brusher};

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
type IndexType = u32;
/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub type VertexBuffers = tess::VertexBuffers<Vertex, IndexType>;

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

/// Zero-sized type used to implement various vertex construction traits from
/// Lyon.
pub struct VertexConstructor<'a> {
    pub brush: &'a Brush,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor<'_> {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y],
            color: self.brush.brush(Vec2::new(vertex.position().x, vertex.position().y)).as_linear_rgba_f32(),
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor<'_> {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y],
            color: self.brush.brush(Vec2::new(vertex.position().x, vertex.position().y)).as_linear_rgba_f32(),
        }
    }
}
