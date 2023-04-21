//see: https://github.com/Chlumsky/msdfgen#using-a-multi-channel-distance-field

#version 330 core
out vec4 FragColor;
uniform vec3 color;
uniform float scale;

uniform sampler2D text_map;
uniform float smoothness;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;



float median(float a, float b, float c) {
  return max(min(a, b), min(max(a, b), c));
}


float w1(float d) {
  return clamp(d/fwidth(d) + 0.5, 0.0, 1.0);
}

float w2(float sd) {
  float screenPxRange = 2.0;
  float screenPxDistance = screenPxRange * (sd - 0.5);
  return clamp(screenPxDistance + 0.5, 0.0, 1.0);

}



float w3(float sd) {
  float screenPxRange = 0.5;
  float screenPxDistance = screenPxRange * (sd - 0.5);
  return clamp(screenPxDistance + 0.5, 0.0, 1.0);

}

void main()
{

  // Bilinear sampling of the distance field
  vec3 s = texture2D(text_map, IN.TexCoords).rgb;

  // Acquire the signed distance
  float sd = median(s.r, s.g, s.b) - 0.5;

  // Weight between inside and outside (anti-aliasing)
  float w = w1(sd);
  FragColor = vec4(color, w);

}
