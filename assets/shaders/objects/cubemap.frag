#version 330 core
out vec4 FragColor;
uniform samplerCube cubemap;

in VS_OUTPUT {
  vec3 TextureDir;
} IN;

void main()
{
  FragColor = texture(cubemap, IN.TextureDir);
}
