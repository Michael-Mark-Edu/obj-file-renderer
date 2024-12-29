#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex;
layout (location = 2) in vec3 normal;

out vec3 vert_pos;
out vec2 vert_tex;
out vec3 vert_normal;

uniform mat4 transform;

void main() {
    // Pass vertex attributes to fragment shader
    vert_pos = pos;
    vert_tex = tex;
    vert_normal = normal;

    // Apply perspective * view matrices
    gl_Position = transform * vec4(pos, 1.0);
}
