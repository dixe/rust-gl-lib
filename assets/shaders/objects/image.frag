#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{
  vec4 col = texture2D(text_map, IN.TexCoords);

  FragColor = vec4(col.xyz, 1.0);

  //FragColor = vec4(vec3(1.0, 1.0, 1.0) - col.xyz, 1.0);


  //FragColor = vec4(IN.TexCoords, 0.0, 1.0);

}
