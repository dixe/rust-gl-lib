#version 330 core
out vec4 FragColor;
uniform vec4 color;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{

  // image is white background black letters in rgb, so inverse the x and we get 1 where there are pixels and 0 where transparent
  float alpha = 1.0 - texture2D(text_map, IN.TexCoords).x;
  FragColor = color * alpha;
}
