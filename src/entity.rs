//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::{ComputedVisibility, GlobalTransform, Handle, Transform, Visibility},
    render::primitives::Aabb,
    sprite::Mesh2dHandle,
};
use lyon_tessellation::{self as tess};

use crate::{prelude::Geometry, render::ShapeMaterial};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mesh: Mesh2dHandle,
    pub aabb: Aabb,
    pub material: Handle<ShapeMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle)]
pub struct SimpleShapeBundle {
    pub path: Path,
    pub mesh: Mesh2dHandle,
    pub material: Handle<ShapeMaterial>,
    pub visibility: Visibility,
}
impl Default for SimpleShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mesh: Mesh2dHandle::default(),
            material: Handle::<ShapeMaterial>::default(),
            visibility: Visibility::Visible,
        }
    }
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mesh: Mesh2dHandle::default(),
            aabb: Aabb::default(),
            material: Handle::<ShapeMaterial>::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Component)]
pub struct Path(pub tess::path::Path);

impl Geometry for Path {
    fn add_geometry(&self, b: &mut tess::path::path::Builder) {
        b.extend_from_paths(&[self.0.as_slice()]);
    }
}
