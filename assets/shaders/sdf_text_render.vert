#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 aTexCoord;


out VS_OUTPUT {
  vec2 TexCoords;
} OUT;


void main()
{
  gl_Position = vec4(pos, 0.0, 1.0);

  OUT.TexCoords = aTexCoord;
}
