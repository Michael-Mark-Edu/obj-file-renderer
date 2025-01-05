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
uniform Material material;

struct DirectionalLight {
  vec3 direction;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

vec3 calc_directional_light(DirectionalLight light) {
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
  vec3 half_angle = normalize(light_dir + camera_dir);
  vec3 specular =
      material.specular * light.specular *
      vec3(texture(material.specular_map, vert_tex)) *
      pow(max(dot(vert_normal, half_angle), 0.0), material.shininess);

  return ambient + diffuse + specular;
}

struct PointLight {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
  float constant;
  float linear;
  float quadratic;
};

vec3 calc_point_light(PointLight light) {
  // Calculate ambient component
  vec3 ambient = material.ambient * light.ambient *
                 vec3(texture(material.ambient_map, vert_tex));

  // Calculate diffuse component
  vec3 light_dir = normalize(light.position - vert_pos);
  vec3 diffuse = material.diffuse * light.diffuse *
                 vec3(texture(material.diffuse_map, vert_tex)) *
                 max(dot(vert_normal, light_dir), 0.0);

  // Calculate specular component
  vec3 camera_dir = normalize(camera_pos - vert_pos);
  vec3 half_angle = normalize(light_dir + camera_dir);
  vec3 specular =
      material.specular * light.specular *
      vec3(texture(material.specular_map, vert_tex)) *
      pow(max(dot(vert_normal, half_angle), 0.0), material.shininess);

  // Calculate attenuation
  float distance = length(light.position - vert_pos);
  float attenuation = 1.0 / (light.constant + light.linear * distance +
                             light.quadratic * distance * distance);

  return attenuation * (ambient + diffuse + specular);
}

void main() {
  DirectionalLight light1 =
      DirectionalLight(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0),
                       vec3(1.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));

  DirectionalLight light2 =
      DirectionalLight(vec3(-1.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0),
                       vec3(0.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0));

  PointLight light3 =
      PointLight(vec3(0.0, 1.5, 0.0), vec3(1.0), vec3(0.0, 0.0, 1.0), vec3(1.0),
                 1.0, 0.22, 0.2);

  vec3 directional_lights =
      calc_directional_light(light1) + calc_directional_light(light2);

  vec3 point_lights = calc_point_light(light3);

  // Sum the components together
  final_color = vec4(directional_lights + point_lights, 1.0);
}
