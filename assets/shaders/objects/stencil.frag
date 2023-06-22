#version 330 core
out vec4 FragColor;


in VS_INPUT {
   flat vec3 Normal;
   vec3 FragPos;
   flat vec3 Color;
} IN;

void main()
{
  FragColor = vec4(0.0, 0.0, 0.0, 1.0);
}
