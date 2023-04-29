#version 330 core

uniform vec4 u_color;

in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform float pixel_width;
uniform float pixel_height;

uniform float radius;


float roundedRectangle(vec2 p, vec2 size, float radius)
{
   return length(max(abs(p) - size + radius,0.0)) - radius;
}



void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // multiply by 2 to get -1.0...1.0 range
    float u = IN.FragPos.x * 2.0;
    float v = IN.FragPos.y * 2.0;


    // uv but in screen space.
    vec2 uv =  vec2(u * pixel_width , v * pixel_height);

    vec4 col = u_color;

    vec2 size = vec2(pixel_width , pixel_height);

    // higher is more blur, and also thicker corners
    float dist = roundedRectangle(uv , size  , radius) ;


    float alpha =  (1.0 - smoothstep(0.0, 1.0, dist));

    col.w = 1.0;

    FragColor = col;
}
