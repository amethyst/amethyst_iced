#version 450

layout (std140, set = 0, binding = 0) uniform TriangleUniform {
    uniform mat4 u_transform;
};

layout(location = 0) in vec2 in_pos;
layout(location = 1) in vec4 in_color;

layout(location = 0) out vec4 o_color;

void main() {
    vec4 transformed = u_transform * vec4(in_pos, 0.0, 1.0); 
    gl_Position = vec4(transformed.xy, 0.0, 1.0); 

    //if (gl_VertexIndex == 0) {
    //    gl_Position = vec4(0.0, -0.5, 0.0, 1.0);
    //} else if(gl_VertexIndex == 1) {
    //    gl_Position = vec4(-0.5, 0.5, 0.0, 1.0);
    //} else if(gl_VertexIndex == 2) {
    //    gl_Position = vec4(0.5, 0.5, 0.0, 1.0);
    //}
    o_color = in_color;
}