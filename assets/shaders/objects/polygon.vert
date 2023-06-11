#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;

out VS_OUTPUT {
  vec4 Color;
} OUT;

void main()
{

  vec4 pos = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
  OUT.Color = vec4(1.0, 0.1, 0.1, 1.0);
  gl_Position = pos;
}
