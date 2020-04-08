#version 450

layout(location = 0) in vec4 triangle_vert_out_color;

layout(location = 0) out vec4 out_color;

void main() {
    out_color = triangle_vert_out_color;
}