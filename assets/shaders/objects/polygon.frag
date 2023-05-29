#version 330 core

out vec4 FragColor;

in VS_OUTPUT {
    vec4 Color;
} IN;

void main()
{
  FragColor = IN.Color;
}
