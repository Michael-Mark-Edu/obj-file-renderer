#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex;
layout (location = 2) in vec3 normal;

out vec3 pos_3d;

uniform mat4 azimuth_matrix;
uniform mat4 elevation_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

void main() {
    pos_3d = pos;
    gl_Position = perspective_matrix * view_matrix * elevation_matrix * azimuth_matrix * vec4(pos, 1.0);
}
