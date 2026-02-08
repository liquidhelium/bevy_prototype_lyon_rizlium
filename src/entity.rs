//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::{
        GlobalTransform, InheritedVisibility, Mesh2d, MeshMaterial2d, Transform, ViewVisibility,
        Visibility,
    },
    utils::default,
};
use lyon_tessellation::{self as tess};

use crate::{prelude::Geometry, render::GradientMaterial};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<GradientMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ViewVisibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
}

#[derive(Bundle)]
pub struct SimpleShapeBundle {
    pub path: Path,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<GradientMaterial>,
    pub visibility: Visibility,
}
impl Default for SimpleShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mesh: Mesh2d::default(),
            material: MeshMaterial2d::<GradientMaterial>::default(),
            visibility: Visibility::Visible,
        }
    }
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mesh: Mesh2d::default(),
            material: MeshMaterial2d::<GradientMaterial>::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ViewVisibility::default(),
            inherited_visibility: default(),
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
