#version 330 core
layout (location = 0) in vec2 aPos;

uniform mat4 transform;

out VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} OUT;

void main()
{
    vec4 pos = transform * vec4(aPos.x, aPos.y, 0.0, 1.0);

    OUT.FragPos = aPos.xy;
    OUT.Pos = aPos.xy;

    gl_Position = pos;
}
