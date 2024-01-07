#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{
  vec4 col = texture2D(text_map, IN.TexCoords);
  //col.xyz = IN.TexCoords.xxx;
  //col.w = 1.0;
  // assume premultiplied in texture
  FragColor = col;
}
