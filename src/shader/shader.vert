#version 450

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 transform;
    vec4 size;
};

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 color;
layout(location = 0) out vec4 out_color;

void main() {
    // Apply transformation in screen coordinates.
    vec4 transformed = transform * vec4(pos, 0.0, 1.0);

    // Convert screen coordinates to vulkan coordinates.
    gl_Position = vec4(
        transformed.xy / size.xy * 2.0 - 1.0,
        transformed.zw
    );

    out_color = color;
}
