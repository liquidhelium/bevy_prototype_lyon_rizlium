//! Render plugin

use bevy::{
    asset::{load_internal_asset, AssetApp, AssetId}, color::{Color, LinearRgba}, prelude::{App, Asset, Assets, Handle, Plugin, Shader, Vec2}, reflect::prelude::*, render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, sprite::{Material2d, Material2dPlugin}
};

/// Handle to the custom shader with a unique random ID
pub const GRADIENT_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(3_191_283_017_262_752_456);

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

        app.world_mut()
            .resource_mut::<Assets<GradientMaterial>>()
            .insert(AssetId::<GradientMaterial>::default(), GradientMaterial::default());
    }
}

impl Material2d for GradientMaterial {
    fn fragment_shader() -> ShaderRef {
        GRADIENT_MATERIAL_SHADER_HANDLE.into()
    }
    fn vertex_shader() -> ShaderRef {
        GRADIENT_MATERIAL_SHADER_HANDLE.into()
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
    pub end_pos: Vec2
}