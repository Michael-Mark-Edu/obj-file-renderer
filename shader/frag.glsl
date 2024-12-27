#version 330 core

in vec3 pos_3d;

out vec4 final_color;

void main() {
    final_color = vec4((pos_3d + 1.0) * 0.5, 1.0);
}
