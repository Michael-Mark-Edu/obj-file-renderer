#version 330 core

uniform ivec2 window_size;
uniform ivec2 mouse_pos;

out vec4 final_color;

void main() {
    final_color = vec4(float(mouse_pos.x) / window_size.x, float(mouse_pos.y) / window_size.y, 0.0, 1.0);
}
