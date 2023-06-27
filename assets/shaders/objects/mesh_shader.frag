#version 330 core
out vec4 Color;

in VS_OUTPUT {
  flat vec3 Normal;
    vec3 FragPos;
    flat vec3 Color;
    vec4 FragPosLightSpace;
} IN;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;
uniform sampler2D Texture;
uniform sampler2D Texture2;
uniform sampler2D shadowMap;


float ShadowCalculation(vec4 fragPosLightSpace, vec3 normal, vec3 lightDir)
{

  // if normal points away from light, we now that it is in shadow
  // this can also eliminate the bias sicne that created notisable
  float angle = dot(normal, lightDir);
  if (angle < 0.0 ) {
    return 1.0;
  }

  // get correct projection, when using perspective and ortho
  // in [-1,1]
  vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;

  // map to [0,1]
  projCoords = projCoords * 0.5 + 0.5;

  // light outside light view frustum z far
  if(projCoords.z > 1.0)
  {
    return 0.0;
  }

  float closestDepth = texture(shadowMap, projCoords.xy).r;
  float currentDepth = projCoords.z;
  float shadow = currentDepth  > closestDepth ? 1.0 : 0.0;

  return closestDepth;
}


void main()
{

  vec3 col = IN.Color;
  //vec3 col = vec3(0.3, 0.9, 0.3);
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

  float shadow = ShadowCalculation(IN.FragPosLightSpace, norm, lightDir);

  Color = vec4((ambient + diffuse + specular) * col, 1.0f);
  //Color = vec4((ambient + (1.0 - shadow) * diffuse + specular) * col, 1.0f);
  //Color = vec4(shadow, shadow, shadow, 1.0);
}
