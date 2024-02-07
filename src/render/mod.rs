//! Render plugin

use bevy::{
    asset::load_internal_asset,
    prelude::{AddAsset, App, Assets, Handle, HandleUntyped, Plugin, Shader, Vec2},
    reflect::{prelude::*, TypeUuid},
    render::{color::Color, render_resource::{AsBindGroup, ShaderRef, ShaderType}},
    sprite::{Material2d, Material2dPlugin},
};

/// Handle to the custom shader with a unique random ID
pub const GRADIENT_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3_191_283_017_262_752_456);

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

        app.world
            .resource_mut::<Assets<GradientMaterial>>()
            .set_untracked(Handle::<GradientMaterial>::default(), GradientMaterial::default());
    }
}

impl Material2d for GradientMaterial {
    fn fragment_shader() -> ShaderRef {
        GRADIENT_MATERIAL_SHADER_HANDLE.typed().into()
    }
    fn vertex_shader() -> ShaderRef {
        GRADIENT_MATERIAL_SHADER_HANDLE.typed().into()
    }
}

/// A simple `Material2d` that renders with vertex colors.
#[derive(Default, AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Default, Debug)]
#[uuid = "ab2e068e-0cca-4941-a114-524af2c431bb"]
pub struct GradientMaterial {
    #[uniform(0)]
    pub(crate) uniform: GradientMaterialUniform,
}

#[derive(ShaderType, Reflect, Default, Debug, Clone, Copy)]
pub struct GradientMaterialUniform {
    pub start: Color,
    pub end: Color,
    pub start_pos: Vec2,
    pub end_pos: Vec2
}