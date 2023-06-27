#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 BoneWeights;
layout (location = 3) in vec2 BoneIndices;

uniform mat4 light_space_mat;
uniform mat4 model;
uniform mat4 uBones[32];


mat4 boneTransform() {

  if(int(BoneIndices.x) < 0)
  {
    return mat4(1.0);
  }
  mat4 ret;

  // Weight1 * Bone1 + Weight2 * Bone2
  ret = BoneWeights.x * uBones[int(BoneIndices.x)]
       + BoneWeights.y * uBones[int(BoneIndices.y)];

  return ret;

}

void main()
{
  mat4 bt = boneTransform();
    gl_Position = light_space_mat * model * bt * vec4(aPos, 1.0);
}
