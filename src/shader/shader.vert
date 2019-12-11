#version 450

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 transform;
    vec4 size;
};

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 color;
layout(location = 0) out vec4 out_color;

void main() {
    gl_Position = /* transform * */ vec4(
        // Convert screen coordinates to vulkan coordinates.
        pos / size.xy * 2.0 - 1.0,
        0.0,
        1.0
    );

    out_color = color;
}
