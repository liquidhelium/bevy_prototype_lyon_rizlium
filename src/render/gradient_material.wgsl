#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_view_bindings::view,
}
#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct GradientMaterialUniform {
    start: vec4<f32>,
    end: vec4<f32>,
    start_pos: vec2<f32>,
    end_pos: vec2<f32>, 
}

@group(2) @binding(0) var<uniform> material: GradientMaterialUniform;

fn invlerp_points(pos1: vec2<f32>, pos2: vec2<f32>, pos: vec2<f32>) -> f32 {
    let pointing_vec = pos2 - pos1;
    let vec_length = length(pointing_vec);
    let posing_vec = pos - pos1;
    if vec_length != 0. {
        let product = dot(pointing_vec, posing_vec);
        return product / (vec_length * vec_length);
    } else {
        return 0.;
    }
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
};
struct VertexOutput {
    // this is `clip position` when the struct is used as a vertex stage output 
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) local_position: vec2<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) uv: vec2<f32>,
    #ifdef VERTEX_TANGENTS
    @location(4) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
    @location(5) color: vec4<f32>,
    #endif
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.local_position = vertex.position.xy;
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_POSITIONS
    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh2d_position_local_to_world(
        model,
        vec4<f32>(vertex.position, 1.0)
    );
    out.position = mesh_functions::mesh2d_position_world_to_clip(out.world_position);
#endif

#ifdef VERTEX_NORMALS
    out.world_normal = mesh_functions::mesh2d_normal_local_to_world(vertex.normal, vertex.instance_index);
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh2d_tangent_local_to_world(
        model,
        vertex.tangent
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
    return out;
}



@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var invlerp_progress = invlerp_points(material.start_pos, material.end_pos, mesh.local_position);
    var color = mix(material.start, material.end, invlerp_progress);
    var output_color: vec4<f32> = color;
    return output_color;
}


