#version 450

layout(set = 1, binding = 0) uniform sampler2D tex;

layout(location = 0) in vec2 uv;
layout(location = 1) in vec4 text_out_color;

layout(location = 0) out vec4 o_color;

void main() {
    vec4 col = texture(tex, uv);
    o_color = vec4(text_out_color.xyz,col.a);
}