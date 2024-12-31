#version 330 core

in vec3 vert_pos;
in vec2 vert_tex;
in vec3 vert_normal;

out vec4 final_color;

uniform vec3 camera_pos;
uniform sampler2D diffuse_map;
uniform sampler2D specular_map;

struct Material {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
  float shininess;
};

struct Light {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

uniform Material material;

// Phong lighting model
void main() {
  Light light = Light(vec3(2.0, 1.2, 1.5), vec3(1.0, 1.0, 0.0),
                      vec3(0.0, 1.0, 1.0), vec3(1.0, 0.0, 1.0));

  // Calculate ambient component
  vec3 ambient = material.ambient * light.ambient;

  // Calculate diffuse component
  vec3 light_dir = normalize(light.position - vert_pos);
  vec3 diffuse = material.diffuse * light.diffuse *
                 vec3(texture(diffuse_map, vert_tex)) *
                 max(dot(vert_normal, light_dir), 0.0);

  // Calculate specular component
  vec3 camera_dir = normalize(camera_pos - vert_pos);
  vec3 reflect_dir = reflect(-light_dir, vert_normal);
  vec3 specular =
      material.specular * light.specular *
      vec3(texture(specular_map, vert_tex)) *
      pow(max(dot(camera_dir, reflect_dir), 0.0), material.shininess);

  // Sum the components together
  final_color = vec4(ambient + diffuse + specular, 1.0);
}
