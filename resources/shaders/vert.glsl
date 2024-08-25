#version 460

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
    mat4 objectToWorld = model;
    mat4 objectToScreen = perspective * view * objectToWorld;

    // transform vertex position to 2D Screen Space + depth
    gl_Position = objectToScreen * vec4(position, 1.0);

    // transform vertex position and normal to an appropriate space for shading calculations
    v_position = (objectToWorld * vec4(position, 1.0)).xyz;
    v_normal = (transpose(inverse(objectToWorld)) * vec4(normal, 0.0)).xyz;

    // pass the uv coordinate
    v_tex_coords = tex_coords;
}