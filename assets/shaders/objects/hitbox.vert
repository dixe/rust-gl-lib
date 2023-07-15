/// Assume that the model matrix has been multipled/set before rendering
/// so positions are in world space and thus we only beed view and projection matrices
#version 330 core
layout (location = 0) in vec3 Position;

uniform mat4 view;
uniform mat4 projection;

void main()
{
  vec4 pos = vec4(Position, 1.0);
  gl_Position =  projection * view * pos;
}
