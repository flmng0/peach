[[location(0)]]
var<in> in_pos: vec2<f32>;
[[location(1)]]
var<in> in_color: vec4<f32>;

[[location(0)]]
var<out> vertex_color: vec4<f32>;

[[builtin(position)]]
var<out> out_pos: vec4<f32>;

[[stage(vertex)]]
fn vs_main() {
    vertex_color = in_color;
    out_pos = vec4<f32>(in_pos, 0.0, 1.0);
}

[[location(0)]]
var<in> in_color: vec4<f32>;

[[location(0)]]
var<out> out_color: vec4<f32>;

[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
}