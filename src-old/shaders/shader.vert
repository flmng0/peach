#version 450

layout(set = 0, binding = 0) uniform Uniforms {
    vec4 size;
    mat4 transform;
};

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 color;
layout(location = 0) out vec4 out_color;

void main() {
    gl_Position = transform * vec4(
        pos / size.xy * 2.0 - 1.0,
        0.0,
        1.0
    );

    out_color = color;
}
