#version 330

in vec2 pos;
in vec2 tex;
out vec2 v_tex_coords;

uniform mat4 target;
uniform mat4 projection;

void main() {
    v_tex_coords = tex;
    gl_Position = projection * target * vec4(pos, 0.0, 1.0);
}
