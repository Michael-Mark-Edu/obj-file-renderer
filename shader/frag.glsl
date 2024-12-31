#version 330 core

in vec3 vert_pos;
in vec2 vert_tex;
in vec3 vert_normal;

out vec4 final_color;

uniform vec3 camera_pos;

struct Material {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
  float shininess;
  sampler2D ambient_map;
  sampler2D diffuse_map;
  sampler2D specular_map;
};

struct DirectionalLight {
  vec3 direction;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

uniform Material material;

// Phong lighting model
void main() {
  DirectionalLight light =
      DirectionalLight(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0),
                       vec3(1.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0));

  // Calculate ambient component
  vec3 ambient = material.ambient * light.ambient *
                 vec3(texture(material.ambient_map, vert_tex));

  // Calculate diffuse component
  vec3 light_dir = normalize(-light.direction);
  vec3 diffuse = material.diffuse * light.diffuse *
                 vec3(texture(material.diffuse_map, vert_tex)) *
                 max(dot(vert_normal, light_dir), 0.0);

  // Calculate specular component
  vec3 camera_dir = normalize(camera_pos - vert_pos);
  vec3 reflect_dir = reflect(-light_dir, vert_normal);
  vec3 specular =
      material.specular * light.specular *
      vec3(texture(material.specular_map, vert_tex)) *
      pow(max(dot(camera_dir, reflect_dir), 0.0), material.shininess);

  // Sum the components together
  final_color = vec4(ambient + diffuse + specular, 1.0);
}
