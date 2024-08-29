#version 460
const int MAX_LIGHT_COUNT = 5;

layout (std140) uniform lightsBlock {
    vec3 light_positions[MAX_LIGHT_COUNT];
    vec3 light_colors[MAX_LIGHT_COUNT];
};

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

uniform sampler2D u_texture;
uniform int u_light_count;

out vec4 color;

const vec3 ambient_color = vec3(0.2, 0.2, 0.2);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    vec3 total_light = vec3(0.0);

    vec3 normal = normalize(v_normal);
    vec3 camera_dir = normalize(-v_position);

    for (int i = 0; i < u_light_count; ++i) {
        vec3 light_dir = normalize(light_positions[i] - v_position);

        float diffuse = max(dot(normal, light_dir), 0.0);
        vec3 half_direction = normalize(light_dir + camera_dir);
        float specular = pow(max(dot(half_direction, normal), 0.0), 16.0);

        vec3 light_color = light_colors[i];

        total_light += light_color * (diffuse + specular);
    }

    vec4 tex_color = texture(u_texture, v_tex_coords);
    color = vec4(total_light, 1.0) * tex_color;
}
