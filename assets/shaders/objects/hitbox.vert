/// Assume that the model matrix has been multipled/set before rendering
/// so positions are in world space and thus we only beed view and projection matrices
#version 330 core
layout (location = 0) in vec3 Position;



out VS_OUTPUT {
  vec3 Color;
} OUT;


uniform mat4 view;
uniform mat4 model;
uniform mat4 projection;

void main()
{
  vec4 pos = vec4(Position, 1.0);
  OUT.Color = Position;
  gl_Position =  projection * view * model * pos;
}
