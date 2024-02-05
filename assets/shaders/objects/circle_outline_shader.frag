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
uniform float thickness;

float circle_outline(vec2 p)
{
  // make two circles
  // first is out regular nagive inside, positive outside, with radius = r
  float c1 = length(p) - radius;

  // second is a smaller circle with negative outside, and positive inside and radius = r - thickness
  float c2 = (radius - thickness) - length(p);

  // there is a small "outline" where both circles are negative, therefore take the max of the two
  return max(c1, c2);
}

void main()
{
  // maybe look at https://www.shadertoy.com/view/WtdSDs
  // or https://iquilezles.org/articles/distfunctions

  // Squaxore is defined with corners in 0.5 and -0.5 on both x and y axis.

  float screen_x = IN.FragPos.x * pixel_width;
  float screen_y = IN.FragPos.y * pixel_height;

  vec4 col = u_color;

  vec2 uv = vec2(screen_x, screen_y);

  float dist = circle_outline(uv);

  float alpha = (1.0 - smoothstep(0.0, 1.0, dist));

  col = col * alpha;

  FragColor = col;

}
