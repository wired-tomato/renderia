#version 330 core
out vec4 FragColor;

in vec3 vColor;
in vec2 uv;

uniform sampler2D texture0;

void main() {
    FragColor = texture(texture0, uv) * vec4(vColor, 1.0);
}