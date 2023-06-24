#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 projection;
uniform mat4 view;

out VS_OUTPUT {
  vec3 TextureDir;
} OUT;


void main()
{
  OUT.TextureDir.x = aPos.x;
  OUT.TextureDir.y = -aPos.z;
  OUT.TextureDir.z = aPos.y;

  // remove translation part of view model
  vec4 pos =  projection * mat4(mat3(view)) * vec4(aPos, 1.0);
  gl_Position = pos.xyww;
}
