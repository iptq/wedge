#version 330

in vec2 point;

uniform mat4 target;
uniform mat4 projection;

void main() {
    gl_Position = projection * target * vec4(point, 0.0, 1.0);
}