struct Uniforms {
    view_proj: mat4x4<f32>,
    eye_pos: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position to clip space
    out.clip_position = uniforms.view_proj * vec4<f32>(in.position, 1.0);

    // Pass world-space data to fragment shader
    out.world_normal = in.normal;
    out.world_position = in.position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional lighting
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let normal = normalize(in.world_normal);

    // Lambertian diffuse
    let diffuse = max(dot(normal, light_dir), 0.0);

    // Ambient
    let ambient = 0.2;

    // Final color (gray material)
    let base_color = vec3<f32>(0.7, 0.7, 0.7);
    let color = base_color * (ambient + diffuse * 0.8);

    return vec4<f32>(color, 1.0);
}
