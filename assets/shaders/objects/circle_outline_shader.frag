#version 330 core

in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform vec4 u_color;

uniform float pixel_width;
uniform float pixel_height;
uniform float radius_outer;
uniform float radius_inner;

float circle_outline(vec2 p, float thickness)
{
  return max((radius_outer - thickness) - length(p), length(p) - radius_outer);
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

  float dist = circle_outline(uv, 1.0);

  float alpha = (1.0 - smoothstep(0.0, 1.0, dist));

  col = col * alpha;

  FragColor = col;

}
