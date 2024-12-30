#version 330 core

in vec3 vert_pos;
in vec2 vert_tex;
in vec3 vert_normal;

out vec4 final_color;

uniform vec3 camera_pos;
uniform sampler2D diffuse_map;
uniform sampler2D specular_map;

// Phong lighting model
void main() {
  vec3 light_color = vec3(1.0, 1.0, 1.0);
  vec3 light_pos = vec3(2.0, 1.2, 1.5);

  // Calculate ambient component
  float ambient_strength = 0.1;
  vec3 ambient = ambient_strength * light_color;

  // Calculate diffuse component
  float diffuse_strength = 1.0;
  vec3 light_dir = normalize(light_pos - vert_pos);
  vec3 diffuse = diffuse_strength * light_color *
                 vec3(texture(diffuse_map, vert_tex)) *
                 max(dot(vert_normal, light_dir), 0.0);

  // Calculate specular component
  float specular_strength = 0.5;
  float shininess = 320.0;
  vec3 camera_dir = normalize(camera_pos - vert_pos);
  vec3 reflect_dir = reflect(-light_dir, vert_normal);
  vec3 specular = specular_strength * light_color *
                  vec3(texture(specular_map, vert_tex)) *
                  pow(max(dot(camera_dir, reflect_dir), 0.0), shininess);

  // Sum the components together
  final_color = vec4(ambient + diffuse + specular, 1.0);
}
