#version 450 core
out vec4 Color;

in VS_OUTPUT {
  vec3 Normal;
  vec3 FragPos;
  vec3 Color;
  vec4 FragPosLightSpace;
  vec2 TexCord;
} IN;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;


layout(binding=0) uniform sampler2D Texture;
layout(binding=1) uniform sampler2D shadowMap;



void main() {

  vec3 col = texture(Texture, IN.TexCord).rgb;

  // for now just use the texture color
  Color = vec4(col, 1.0);

}
