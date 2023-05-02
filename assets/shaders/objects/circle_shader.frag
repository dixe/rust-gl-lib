#version 330 core


in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform float pixel_width;
uniform float pixel_height;

uniform float radius;

uniform float color_scale;



float circle(vec2 p, float radius)
{
  return length(p) - radius;
}

void main()
{

  // maybe look at https://www.shadertoy.com/view/WtdSDs
  // or https://iquilezles.org/articles/distfunctions

  // Square is defined with corners in 0.5 and -0.5 on both x and y axis.

  float screen_x = IN.FragPos.x * pixel_width;
  float screen_y = IN.FragPos.y * pixel_height;

  vec3 col = vec3(.8, 0.8, .8) * color_scale;
  vec2 uv = vec2(screen_x, screen_y);

  float dist = circle(uv, radius);

  float alpha = (1.0 - smoothstep(0.0, 1.0, dist));

  col = col * alpha;
  FragColor = vec4(col, alpha);

}
