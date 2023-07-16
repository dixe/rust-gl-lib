#version 450 core
out vec4 Color;

in VS_OUTPUT {
  vec3 Color;
} IN;


void main()
{
  Color = vec4(abs(IN.Color), 1.0);
}
