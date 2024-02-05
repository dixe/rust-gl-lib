#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in mat4 aInstanceMatrix;
layout (location = 5) in vec4 aColor;

uniform mat4 transform;

out VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
    vec4 color;
} OUT;

void main()
{
    vec4 pos = aInstanceMatrix * vec4(aPos.x, aPos.y, 0.0, 1.0);

    OUT.FragPos = aPos.xy;
    OUT.Pos = aPos.xy;
    OUT.color = aColor.xyzw;

    gl_Position = pos;
}
