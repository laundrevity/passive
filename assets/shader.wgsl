// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) instance_pos: vec3<f32>,
    @location(3) theta: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;

    let rotation_matrix = mat2x2<f32>(
        cos(instance.theta), -sin(instance.theta),
        sin(instance.theta),  cos(instance.theta)
    );

    let rotated_position = rotation_matrix * model.position.xy;
    out.clip_position = vec4<f32>(vec3<f32>(rotated_position, model.position.z) + instance.instance_pos, 1.0);

    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}