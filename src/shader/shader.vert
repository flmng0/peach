#version 450

#define MAX_STATE_STACK 16

layout(set = 0, binding = 0) uniform Transforms {
    mat4 transforms[MAX_STATE_STACK];
};

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 color;
layout(location = 2) in uint index;

layout(location = 0) out vec4 out_color;

void main() {
    mat4 transform = transforms[index];
    gl_Position = transform * vec4(pos, 0.0, 1.0);

    out_color = color;
}
