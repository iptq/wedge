#version 330

in vec4 pos;
out vec4 outcolor;

uniform vec4 color;

const float threshold = 0.05;

const vec4 top = vec4(0.5, 0.5, 0.5, 1.0);
const vec4 bot = vec4(0.4, 0.4, 0.4, 1.0);

void main() {
	outcolor = 0.2 * (bot * (1 - pos.y) + top * (1 - pos.x)) + 0.2; 
	// if ((pos.x > -threshold && pos.x < threshold)
	// 	|| (pos.y > -threshold && pos.y < threshold)
	// 	|| (pos.x > 1.0-threshold && pos.x < 1.0+threshold)
	// 	|| (pos.y > 1.0-threshold && pos.y < 1.0+threshold)) {
	// 	outcolor = vec4(0.0, 0.0, 0.0, 1.0);
	// }
}