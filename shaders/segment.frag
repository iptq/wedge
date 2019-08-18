#version 330

in vec2 v_tex_coords;
out vec4 outcolor;

uniform sampler2D tex;
uniform vec4 tint;

void main() {
    outcolor = tint * texture(tex, v_tex_coords);
}
