//see: https://github.com/Chlumsky/msdfgen#using-a-multi-channel-distance-field

#version 330 core
out vec4 FragColor;
uniform vec3 color;

uniform sampler2D text_map;


in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{

  float s = texture2D(text_map, IN.TexCoords).r;

  FragColor = vec4(color, 1.0) * s;
}
