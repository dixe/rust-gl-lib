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

  // Distance from the edge.
  // [0.0, 0.5[ is outside
  // ]0.5;1] is inside
  // And 0.5 is right on the edge
  float dist = texture(text_map, IN.TexCoords).a;


  // Just scale everything below 0.5 (ouside) to 0 and everything inside to 1s
  float u_buffer = 0.5;

  // allow some smoothing for AA at edges
  float smoothing = 0.125 / (smoothness *  scale);

  float alpha = smoothstep(u_buffer - smoothing, u_buffer + smoothing , dist);

  if( alpha < 0.01)
    {
      //alpha = 0.3;
      discard;
    }



  vec3 rgb = texture(text_map, IN.TexCoords).rgb;

  vec3 bgcol = color - vec3(1.0);
  //vec3 bgcol = vec3(1.0) - colo;r

  vec3 aa_col =  vec3(color * alpha + bgcol * (1.0 - alpha));
  //vec3 aa_col =  vec3(color * alpha);
  //vec3 aa_col =  vec3(bgcol * ( alpha));

  FragColor = vec4(aa_col, 1.0);
}
