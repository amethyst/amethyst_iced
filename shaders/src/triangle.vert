#version 450

layout (std140, set = 0, binding = 0) uniform IcedUniform {
    vec2 inverse_window_size;
};

layout(location = 0) in vec2 in_pos;
layout(location = 1) in vec4 in_color;

layout(location = 0) out vec4 triangle_vert_out_color;

void main() {
    gl_Position = vec4(in_pos, 0.0, 1.0);
    triangle_vert_out_color = in_color;
}