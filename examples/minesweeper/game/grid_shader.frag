#version 330 core


in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform float width;
uniform float height;


float grid(vec2 fragCoord, float space_x, float space_y, float gridWidth)
{
    vec2 p  = fragCoord - vec2(.5);
    vec2 size = vec2(gridWidth - .5);


    vec2 a1 = vec2(0.);
    a1.x = mod(p.x - size.x, space_x);
    a1.y = mod(p.y - size.y, space_y);

    vec2 a2 = vec2(0.);
    a2.x = mod(p.x + size.x, space_x);
    a2.y = mod(p.y + size.y, space_y);


    vec2 a = a2 - a1;

    float g = min(a.x, a.y);
    return clamp(g, 0., 1.0);
}

float border(vec2 fragCoord, float gridWidth)
{
    if(fragCoord.x < gridWidth * 2.0 || fragCoord.x > (width - gridWidth * 2.0) ||
    fragCoord.y < gridWidth * 2.0 || fragCoord.y > (height - gridWidth * 2.0))
    {
        return 0.0;
    }
    return 1.0;
}

vec3 base_color(vec2 fragCoord) {
   return vec3(0.8, 0.8, 0.8);
}

void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // add 0.5 to get range of 0..1
    float u = IN.FragPos.x + 0.5;
    float v = IN.FragPos.y + 0.5;

    vec2 fragCoord = vec2(u * width, v* height);


    vec3 col = base_color(fragCoord);
    float space_x = width / 9.;
    float space_y = height / 9.;
    float grid_width = 2.0;
    col *= border(fragCoord, grid_width) * grid(fragCoord, space_x, space_y, grid_width);

    float alpha = 1.0;
    FragColor = vec4(col, alpha);
}
