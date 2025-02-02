//! Contains the plugin and its helper types.
//!
//! The [`ShapePlugin`] provides the creation of shapes with minimal
//! boilerplate.
//!
//! ## How it works
//! The user spawns a [`ShapeBundle`](crate::entity::ShapeBundle) from a
//! system in the `UPDATE` stage.
//!
//! Then, in [`Stage::Shape`] stage, there is a system
//! that creates a mesh for each entity that has been spawned as a
//! `ShapeBundle`.

use bevy::{
    app::{App, Plugin}, asset::{Assets, Handle}, color::palettes::css::FUCHSIA, ecs::{
        query::{Changed, Or}, schedule::IntoSystemSetConfigs, system::{Query, ResMut, Resource}
    }, log::error, prelude::{
        Color, Deref, DerefMut, IntoSystemConfigs, PostUpdate, SystemSet,
    }, render::{
        mesh::{Indices, Mesh, Mesh2d}, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology
    }, sprite::MeshMaterial2d
};
use lyon_tessellation::{self as tess, BuffersBuilder};

use crate::{
    brush::{Brush, Gradient, GradientStop, LinearGradient}, draw::{Fill, Stroke}, entity::Path, render::{GradientMaterial, GradientMaterialPlugin}, vertex::{VertexBuffers, VertexConstructor}
};

/// A plugin that provides resources and a system to draw shapes in Bevy with
/// less boilerplate.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = lyon_tessellation::FillTessellator::new();
        let stroke_tess = lyon_tessellation::StrokeTessellator::new();
        app.insert_resource(FillTessellator(fill_tess))
            .insert_resource(StrokeTessellator(stroke_tess))
            .configure_sets(
                PostUpdate,
                (BuildShapes,).after(bevy::transform::TransformSystem::TransformPropagate),
            )
            .add_systems(PostUpdate, mesh_shapes_system.in_set(BuildShapes))
            .add_plugins(GradientMaterialPlugin)
            .register_type::<Fill>()
            .register_type::<Stroke>()
            .register_type::<Gradient>()
            .register_type::<GradientStop>()
            .register_type::<Brush>()
            .register_type::<LinearGradient>();
    }
}

/// [`SystemSet`] for the system that builds the meshes for newly-added
/// or changed shapes. Resides in [`PostUpdate`] schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct BuildShapes;

/// Queries all the [`ShapeBundle`]s to mesh them when they are added
/// or re-mesh them when they are changed.
#[allow(clippy::type_complexity)]
fn mesh_shapes_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut gradients: ResMut<Assets<GradientMaterial>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (
            Option<&Fill>,
            Option<&Stroke>,
            &Path,
            &mut Mesh2d,
            &mut MeshMaterial2d<GradientMaterial>,
        ),
        Or<(Changed<Path>, Changed<Fill>, Changed<Stroke>)>,
    >,
) {
    for (maybe_fill_mode, maybe_stroke_mode, path, mut mesh, mut material) in &mut query {
        let mut buffers = VertexBuffers::new();

        if let Some(fill_mode) = maybe_fill_mode {
            fill(&mut fill_tess, &path.0, fill_mode, &mut buffers);
        }

        if let Some(stroke_mode) = maybe_stroke_mode {
            stroke(&mut stroke_tess, &path.0, stroke_mode, &mut buffers);
        }

        if (maybe_fill_mode, maybe_stroke_mode) == (None, None) {
            fill(
                &mut fill_tess,
                &path.0,
                &Fill::brush(Color::from(FUCHSIA)),
                &mut buffers,
            );
        }

        mesh.0 = meshes.add(build_mesh(&buffers));
        // fill 与 stroke 可以兼得，但我懒了() 
        if let Some(mode) = maybe_fill_mode {
            *material = bevy::prelude::MeshMaterial2d(gradients.add(GradientMaterial {
                uniform: mode.brush.clone_as_uniform(),
            }));
        }
        else if let Some(mode) = maybe_stroke_mode {
            *material = bevy::prelude::MeshMaterial2d(gradients.add(GradientMaterial {
                uniform: mode.brush.clone_as_uniform(),
            }));
        }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &FillOptions
fn fill(
    tess: &mut ResMut<FillTessellator>,
    path: &tess::path::Path,
    mode: &Fill,
    buffers: &mut VertexBuffers,
) {
    if let Err(e) = tess.tessellate_path(
        path,
        &mode.options,
        &mut BuffersBuilder::new(buffers, VertexConstructor { brush: &mode.brush }),
    ) {
        error!("FillTessellator error: {:?}", e);
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &StrokeOptions
fn stroke(
    tess: &mut ResMut<StrokeTessellator>,
    path: &tess::path::Path,
    mode: &Stroke,
    buffers: &mut VertexBuffers,
) {
    if let Err(e) = tess.tessellate_path(
        path,
        &mode.options,
        &mut BuffersBuilder::new(buffers, VertexConstructor { brush: &mode.brush }),
    ) {
        error!("StrokeTessellator error: {:?}", e);
    }
}

fn build_mesh(buffers: &VertexBuffers) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_indices(Indices::U32(buffers.indices.clone()));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| {
                [v.position[0], v.position[1], 0.0]
            })
            .collect::<Vec<[f32; 3]>>(),
    );

    mesh
}

#[derive(Resource, Deref, DerefMut)]
struct FillTessellator(lyon_tessellation::FillTessellator);

#[derive(Resource, Deref, DerefMut)]
struct StrokeTessellator(lyon_tessellation::StrokeTessellator);
