#version 450 core
out vec4 Color;

in VS_OUTPUT {
  vec3 Normal;
  vec3 FragPos;
  vec3 Color;
  vec4 FragPosLightSpace;
  vec2 TexCord;
} IN;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;
layout(binding=0) uniform sampler2D Texture;
layout(binding=1) uniform sampler2D shadowMap;


float ShadowCalculation(vec4 fragPosLightSpace, vec3 normal, vec3 lightDir)
{

  // if normal points away from light, we now that it is in shadow
  // this can also eliminate the bias since that created notisable
  float angle = dot(normal, lightDir);
  if (angle < 0.0 ) {
    return 1.0; // should be 1
  }

  // get correct projection, when using perspective and ortho
  // in [-1,1]
  vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;

  // map to [0,1]
  projCoords = projCoords * 0.5 + 0.5;

  // light outside light view frustum z far
  if(projCoords.z > 1.0)
  {
    return 0.0; // 0 for light, 1 for dark
  }

  // Outside light square
  if (projCoords.x > 1.0 || projCoords.x  < 0.0 ||
      projCoords.y > 1.0 || projCoords.y  < 0.0)
  {
    return 0.0; // 0 for light, 1 for dark
  }

  float closestDepth = texture(shadowMap, projCoords.xy).r;
  float currentDepth = projCoords.z;
  float shadow = currentDepth > closestDepth ? 1.0 : 0.0;

  return shadow;
}

float calc_light() {
  // ABIENT
  float ambientStrength = 0.5;
  vec3 ambient = ambientStrength * lightColor;

  //DIFFUSE
  vec3 norm = normalize(IN.Normal);
  vec3 lightDir = normalize(lightPos - IN.FragPos);
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = (diff * lightColor) * 0.70;

  // SPECULAR
  float specularStrength = 0.1;
  vec3 viewDir = normalize(viewPos - IN.FragPos);
  vec3 reflectionDir = reflect(-lightDir, IN.Normal);
  float spec = pow(max(dot(viewDir, reflectionDir), 0.0), 5);
  vec3 specular = specularStrength * spec * lightColor;

  // SHADOW
  float shadow = ShadowCalculation(IN.FragPosLightSpace, norm, lightDir);
  //shadow = 0.0;
  float l = ambientStrength + (1.0 - shadow) * (diff + spec);

  return smoothstep(0.50, 0.75, l);

}

void main()
{

  vec3 col = texture(Texture, IN.TexCord).rgb;

  float light = calc_light();


  Color = vec4(light * col, 1.0);

}
