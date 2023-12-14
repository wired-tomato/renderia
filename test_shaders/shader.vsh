#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 uvIn;

out vec3 vColor;
out vec2 uv;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    vColor = color;
    uv = uvIn;
}