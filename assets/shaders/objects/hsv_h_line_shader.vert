#version 330 core
layout (location = 0) in vec2 aPos;

uniform mat4 transform;

out VS_OUTPUT {
  float h;
} OUT;

void main()
{
  // assume that aPos.x is -0.5 ot 0.5, map that to 0..360
  vec4 pos = transform * vec4(aPos.x, aPos.y, 0.0, 1.0);
  OUT.h = (aPos.x + 0.5) * 360.0;
  gl_Position = pos;
}
