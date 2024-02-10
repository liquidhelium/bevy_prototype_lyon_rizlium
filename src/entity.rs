//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::{ViewVisibility, GlobalTransform, Handle, Transform, Visibility},
    render::primitives::Aabb,
    sprite::Mesh2dHandle,
};
use lyon_tessellation::{self as tess};

use crate::{prelude::Geometry, render::GradientMaterial};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mesh: Mesh2dHandle,
    pub aabb: Aabb,
    pub material: Handle<GradientMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ViewVisibility,
}

#[derive(Bundle)]
pub struct SimpleShapeBundle {
    pub path: Path,
    pub mesh: Mesh2dHandle,
    pub material: Handle<GradientMaterial>,
    pub visibility: Visibility,
}
impl Default for SimpleShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mesh: Mesh2dHandle::default(),
            material: Handle::<GradientMaterial>::default(),
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
            material: Handle::<GradientMaterial>::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ViewVisibility::default(),
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
