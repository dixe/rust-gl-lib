#version 330 core
layout (location = 0) in vec3 pos;

uniform mat4 projection;
uniform mat4 view;

out VS_OUTPUT {
  vec3 TextureDir;
} OUT;


void main()
{
  OUT.TextureDir.x = pos.x;
  OUT.TextureDir.y = -pos.z;
  OUT.TextureDir.z = pos.y;

  // remove translation part of view model
  gl_Position = projection * mat4(mat3(view)) * vec4(pos, 1.0);
}
