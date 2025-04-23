#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 transform;
uniform float zoom;

out VS_OUTPUT {
  vec2 TexCoords;
} OUT;


void main()
{
  gl_Position = transform * vec4(pos, 1.0);


  mat3 zoom_mat = mat3(1.0);


  zoom_mat[0][0] = zoom;
  zoom_mat[1][1] = zoom;



  // move center to midle
  mat3 trans1 = mat3(1.0);

  trans1[2][0] = -0.4f;
  trans1[2][1] = -0.4f;


  // move center back after zoom
  mat3 trans2 = mat3(1.0);

  trans2[2][0] = 0.4f;
  trans2[2][1] = 0.4f;

  mat3 trans = trans2 * zoom_mat* trans1;


  vec3 tex = trans * vec3(aTexCoord, 1.0);


  OUT.TexCoords =  tex.xy;
}
