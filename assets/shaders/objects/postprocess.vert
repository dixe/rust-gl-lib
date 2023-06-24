#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 transform;

out VS_OUTPUT {
  vec2 TexCoords;
} OUT;


void main()
{
  vec4 pos =  transform * vec4(aPos, 1.0);

  gl_Position = pos;
  OUT.TexCoords = aTexCoord;
}
