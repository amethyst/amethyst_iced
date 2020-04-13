#version 450

layout (std140, set = 0, binding = 0) uniform TextUniform {
    uniform mat4 u_transform;
};

layout(location = 0) in vec2 in_pos;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec4 in_color;

layout(location = 0) out vec2 uv;
layout(location = 1) out vec4 text_out_color;

void main() {
    text_out_color = in_color;
    uv = in_uv; 

    vec4 transformed = u_transform * vec4(in_pos, 0.0, 1.0); 
    gl_Position = vec4(transformed.xy, 0.0, 1.0); 
}