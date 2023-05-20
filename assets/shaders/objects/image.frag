#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{
  FragColor = texture2D(text_map, IN.TexCoords);
}
