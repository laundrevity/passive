// Vertex shader

struct Uniforms {
    aspect_ratio: f32,
};

@group(1) @binding(0)
var<uniform> u_uniforms: Uniforms;

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
    let aspect_ratio_corrected_position = vec2<f32>(
        rotated_position.x / u_uniforms.aspect_ratio,
        rotated_position.y
    );

    let scaled_instance_position = vec3<f32>(
        instance.instance_pos.x / u_uniforms.aspect_ratio,
        instance.instance_pos.y,
        instance.instance_pos.z
    );
    
    out.clip_position = vec4<f32>(vec3<f32>(aspect_ratio_corrected_position, model.position.z) + scaled_instance_position, 1.0);

    // out.clip_position = vec4<f32>(model.position, 1.0);

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