#version 140

in vec2 v_tex_coords;
in vec4 v_color;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = v_color * texture(tex, v_tex_coords);
    if (color.a < 0.1) discard;
}