#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 transform;

out VS_OUTPUT {
  vec2 TexCoords;
} OUT;


void main()
{
  gl_Position = transform * vec4(pos, 1.0);

  OUT.TexCoords = aTexCoord;
}
