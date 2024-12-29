#version 330 core

in vec3 vert_pos;
in vec2 vert_tex;
in vec3 vert_normal;

out vec4 final_color;

uniform vec3 camera_pos;

// Phong lighting model
void main() {
    vec3 light_color = vec3(1.0, 1.0, 1.0);
    vec3 light_pos = vec3(2.0, 1.2, 1.5);
    vec3 obj_color = vec3(0.0, 1.0, 1.0);

    // Calculate ambient component
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    // Calculate diffuse component
    float diffuse_strength = 1.0;
    vec3 light_dir = normalize(light_pos - vert_pos);
    vec3 diffuse = diffuse_strength * light_color * max(dot(vert_normal, light_dir), 0.0);

    // Calculate specular component
    float specular_strength = 0.5;
    float shininess = 32.0;
    vec3 camera_dir = normalize(camera_pos - vert_pos);
    vec3 reflect_dir = reflect(-light_dir, vert_normal);
    vec3 specular = specular_strength * light_color * pow(max(dot(camera_dir, reflect_dir), 0.0), shininess);

    // Sum the components together
    vec3 result = (ambient + diffuse + specular) * obj_color;
    final_color = vec4(result, 1.0);
}
