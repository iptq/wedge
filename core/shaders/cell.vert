#version 330

in vec2 point;
out vec4 pos;

uniform mat4 target;
uniform mat4 projection;

void main() {
    pos = vec4(point, 0.0, 1.0);
    gl_Position = projection * target * pos;
}
