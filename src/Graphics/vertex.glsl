#version 140

in vec3 position;
in vec2 tex_coords;
in vec4 color;

out vec2 v_tex_coords;
out vec4 v_color;

uniform mat4 matrix;
uniform mat4 vp;

void main() {
    gl_Position = vp * matrix * vec4(position.xyz, 1.0);

    v_color = color;
    v_tex_coords = tex_coords;
}