struct VertexOutput {
    [[location(0)]] color: vec4<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[block]]
struct Uniforms {
    normalize: mat4x4<f32>;
};
[[group(0), binding(0)]] var uniforms: Uniforms;


[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec2<f32>,
    [[location(1)]] color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    out.color = color;
    out.position = uniforms.normalize * vec4<f32>(position, 0.0, 1.0);

    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
