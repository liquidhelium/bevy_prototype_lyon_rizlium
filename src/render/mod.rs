//! Render plugin

use bevy::{
    asset::{load_internal_asset, AssetApp, AssetId},
    color::LinearRgba,
    prelude::{App, Asset, Assets, Handle, Plugin, Shader, Vec2},
    reflect::prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};
use lyon_algorithms::geom::euclid::approxeq::ApproxEq;

/// Handle to the custom shader with a unique random ID
pub const GRADIENT_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    bevy::asset::uuid_handle!("00000000-0000-0000-0000-000000000001");

/// Plugin that provides a custom material for rendering [`Shape`]s
pub struct GradientMaterialPlugin;

impl Plugin for GradientMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GRADIENT_MATERIAL_SHADER_HANDLE,
            "gradient_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<GradientMaterial>::default())
            .register_asset_reflect::<GradientMaterial>();

        // Ignore the previous value; we only need to ensure a default material exists for the handle.
        let _ = app
            .world_mut()
            .resource_mut::<Assets<GradientMaterial>>()
            .insert(
                AssetId::<GradientMaterial>::default(),
                GradientMaterial::default(),
            );
    }
}

impl Material2d for GradientMaterial {
    fn fragment_shader() -> ShaderRef {
        GRADIENT_MATERIAL_SHADER_HANDLE.into()
    }

    fn vertex_shader() -> ShaderRef {
        GRADIENT_MATERIAL_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        if self.uniform.start.alpha.approx_eq_eps(&1.0, &0.01)
            && self.uniform.end.alpha.approx_eq_eps(&1.0, &0.01)
        {
            AlphaMode2d::Opaque
        } else {
            AlphaMode2d::Blend
        }
    }
}

/// A simple `Material2d` that renders with vertex colors.
#[derive(Default, AsBindGroup, Reflect, Debug, Clone, Asset)]
#[reflect(Default, Debug)]
pub struct GradientMaterial {
    #[uniform(0)]
    pub(crate) uniform: GradientMaterialUniform,
}

#[derive(ShaderType, Reflect, Default, Debug, Clone, Copy)]
pub struct GradientMaterialUniform {
    pub start: LinearRgba,
    pub end: LinearRgba,
    pub start_pos: Vec2,
    pub end_pos: Vec2,
}
