
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

void main()
{

  float s = texture2D(text_map, IN.TexCoords).a;

  FragColor = vec4(color * s, 1.0 * s);
}

/*
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

void main()
{

  // Bilinear sampling of the distance field
  vec4 s = texture2D(text_map, IN.TexCoords);

  // Acquire the signed distance
  float sd = s.r - 0.5;

  // Weight between inside and outside (anti-aliasing)

  float w = sd/fwidth(sd) + 0.5;
  w = clamp(w, 0.0, 1.0);



  FragColor = vec4(color, w);
  //FragColor = vec4(color, s.b);
  //FragColor = vec4(color, 1.0);
  /*
  sd = s.a - 0.4;
  w = sd/fwidth(sd) + 0.5;
  w = clamp(w, 0.0, 1.0);

  FragColor = vec4(color, w);

  //FragColor = vec4(0.0, 0.0, 0.0, s.a);

}
*/
