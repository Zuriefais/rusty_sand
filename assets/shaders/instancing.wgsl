#import bevy_sprite::{mesh2d_functions as mesh_functions}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;
    var out: VertexOutput;

    // Assuming mesh_functions::get_model_matrix and mesh_functions::mesh2d_position_local_to_clip
    // are correctly implemented for 2D
    var model = mesh_functions::get_model_matrix(0u);
    out.clip_position = mesh_functions::mesh2d_position_local_to_clip(
        model,
        vec4<f32>(position, 1.0)
    );
    out.color = vertex.i_color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}