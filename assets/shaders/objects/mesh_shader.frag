#version 330 core

in VS_OUTPUT {
  flat vec3 Normal;
    vec3 FragPos;
    flat vec3 Color;
} IN;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;
out vec4 Color;



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


  Color = vec4( (ambient + diffuse + specular) * col, 1.0f);
  //Color = vec4(1.0);
}
