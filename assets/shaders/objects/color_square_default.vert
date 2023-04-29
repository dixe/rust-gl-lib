#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;

uniform mat4 transform;

out VS_OUTPUT {
  vec4 Color;
} OUT;

void main()
{
    vec4 pos = transform * vec4(aPos.x, aPos.y, 0.0, 1.0);
    OUT.Color = aColor;
    gl_Position = pos;
}
