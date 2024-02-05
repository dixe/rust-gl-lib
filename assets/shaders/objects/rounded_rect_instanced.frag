#version 330 core

in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
    vec4 color;
} IN;

out vec4 FragColor;

//uniform float pixel_width;
//uniform float pixel_height;

uniform float radius;


// returns 0 when inside rect, and above when outside
float roundedRectangle(vec2 p, vec2 size, float radius)
{
  return length(max(abs(p) - size + radius, 0.0)) - radius;
}


void main()
{


  float pixel_width = 1.0;
  float pixel_height = 1.0;

  // maybe look at https://www.shadertoy.com/view/WtdSDs

  // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
  // multiply by 2 to get -1.0...1.0 range
  float u = IN.FragPos.x * 2.0 ;
  float v = IN.FragPos.y * 2.0;

  float r = min(radius, min(pixel_width/2.0, pixel_height/2.0));

  // uv but in screen space. uv.u(x) is in [-pixel_width; pixel_width]
  vec2 uv =  vec2(u * pixel_width, v * pixel_height);

  vec4 col = IN.color;

  vec2 size = vec2(pixel_width, pixel_height);

  // higher is more blur, and also thicker corners
  float dist = roundedRectangle(uv, size, r * 2.0);


  float alpha = (1.0 - smoothstep(0.0, 1.0, dist));

  col *= alpha;


  FragColor = col;
  //FragColor = vec4(abs(uv.x / pixel_width), abs(uv.y / pixel_height), 1.0, 1.0);

  //FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
