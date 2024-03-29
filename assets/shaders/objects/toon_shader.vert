#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 BoneWeights;
layout (location = 3) in vec2 BoneIndices;
layout (location = 4) in vec2 TexCord;

out VS_OUTPUT {
  vec3 Normal;
  vec3 FragPos;
  vec3 Color;
  vec4 FragPosLightSpace;
  vec2 TexCord;
} OUT;


uniform mat4 uBones[32];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform sampler2D Texture;
uniform mat4 lightSpaceMat;

mat4 boneTransform() {

  if(int(BoneIndices.x) < 0)
  {
    return mat4(1.0);
  }
  mat4 ret;

  // Weight1 * Bone1 + Weight2 * Bone2
  ret = BoneWeights.x * uBones[int(BoneIndices.x)] + BoneWeights.y * uBones[int(BoneIndices.y)];

  return ret;

}


void main()
{
    mat4 bt = boneTransform();
    vec4 pos = model * bt * vec4(Position, 1.0);


    OUT.FragPos = vec3(pos);
    // This is called normal matrix, maybe do on cpu(the transpose and invere part)
    // and send it in as a uniform
    OUT.Normal = mat3(transpose(inverse(model * bt))) * Normal;

    OUT.FragPosLightSpace = lightSpaceMat * vec4(OUT.FragPos, 1.0);

    OUT.Color = texture(Texture, TexCord).rgb;
    OUT.TexCord = TexCord;
    //OUT.Color = vec3(0.9, 0.7, 0.2);

    gl_Position =  projection * view * pos;
}
