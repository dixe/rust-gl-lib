#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;
uniform float time; // effect elapsed time



const float offset = 1.0 / 300.0;
in VS_OUTPUT {
  vec2 TexCoords;
} IN;



float gray(vec3 c) {
  return 0.2126 * c.r + 0.7152 * c.g + 0.0722 * c.b;

}


void outline () {
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right
    );


    float kernel[9] = float[](
        1, 1, 1,
        1,  -8, 1,
        1, 1, 1
    );

    float sampleTex[9];
    for(int i = 0; i < 9; i++)
    {
      // grayscale
      float c = gray(vec3(texture(text_map, IN.TexCoords.st + offsets[i])));
      sampleTex[i] = c;
    }

    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++) {
      col += sampleTex[i] * kernel[i];
      }

    float r = col.r;

    col = vec3(1.0);

    if(r >= 0.1) {
      FragColor = vec4(1.0 - col, 1.0) + texture(text_map, IN.TexCoords) * 0.1;
    } else {
      FragColor = texture(text_map, IN.TexCoords);
    }
}

// Swirl effect parameters
uniform float radius = 150.0;
uniform float angle = 0.9;
uniform vec2 center_swirl = vec2(800.0, 300.0);
vec4 Swirl(vec2 uv, float time)
{
  float rt_w = 1200;
  float rt_h = 800;

  vec2 texSize = vec2(rt_w, rt_h);
  // pixel coord of current fragment
  vec2 tc = uv * texSize;
  // offset closer to center_swirl
  tc -= center_swirl;
  float dist = length(tc);
  if (dist < radius)
  {
    float percent = (radius - dist) / radius;
    float theta = percent * percent * angle * 8.0;
    float s = sin(theta);
    float c = cos(theta);
    tc = vec2(dot(tc, vec2(c, -s)), dot(tc, vec2(s, c)));
  }
  tc += center_swirl;
  vec3 color = texture2D(text_map, tc / texSize).rgb;
  return vec4(color, 1.0);
}

uniform sampler2D sceneTex; // 0
uniform vec2 center_shock = vec2(0.5, 0.5);

uniform vec3 shockParams = vec3(10.0, 0.8, 0.08);

vec4 ShockWave() {
  vec2 uv = IN.TexCoords.st;
  vec2 texCoord = uv;

  float dist = distance(uv, center_shock);
  // if uv is withing the wave
  if ( (dist <= (time + shockParams.z)) &&
       (dist >= (time - shockParams.z)) )
  {
    float diff = (dist - time);
    float powDiff = 1.0 - pow(abs(diff*shockParams.x),
                                shockParams.y);
    float diffTime = diff  * powDiff;
    vec2 diffUV = normalize(uv - center_shock);
    texCoord = uv + (diffUV * diffTime);
  }
  return texture2D(sceneTex, texCoord);
}

void main()
{
  //FragColor = Swirl(IN.TexCoords.st, time);
  //FragColor = ShockWave();
  outline();

  FragColor =  texture(text_map, IN.TexCoords);//vec4(IN.TexCoords, 0.0, 1.0);
}
