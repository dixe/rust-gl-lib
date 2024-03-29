#version 330 core

in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform vec4 u_color;

uniform float pixel_width;
uniform float pixel_height;
uniform float radius;



float circle(vec2 p, float radius)
{
  return length(p) - radius; // when inside circle output < 0 when outside output > 0
}

void main()
{

  // maybe look at https://www.shadertoy.com/view/WtdSDs
  // or https://iquilezles.org/articles/distfunctions

  // Square is defined with corners in 0.5 and -0.5 on both x and y axis.

  float screen_x = IN.FragPos.x * pixel_width;
  float screen_y = IN.FragPos.y * pixel_height;

  vec4 col = u_color;

  vec2 uv = vec2(screen_x, screen_y);

  float dist = circle(uv, radius);

  float alpha = (1.0 - smoothstep(0.0, 1.0, dist));

  col = col * alpha;

  FragColor = col;

}
