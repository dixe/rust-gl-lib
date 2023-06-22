//// ONLY WORKS FOR SMOOTH SHADING MODELS, WHERE NORMALS FOR THE SAME VERTICES POINTS IN THE SAME DIRECTIONS.
//// IF FLAT SHADES FOR EACH VERTEX/FACE THERE WILL BE A DIFFERENT NORMAL, POINTING IN THE DIRECTION
//// OF THE FACE. WHEN EXTENDING POSITIONS ALONG THOSE NORMALS, WE END UP WITH GAPS
////
#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 BoneWeights;
layout (location = 3) in vec2 BoneIndices;
layout (location = 4) in vec2 TexCord;

uniform mat4 uBones[32];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

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

  // simple version with just extrude along normal, normal has to be smoothshading normal
  // should be calculated when loading gltf and set as a new vec3 vertex attribute


  gl_Position = projection * view * model * bt * vec4(Position + Normal * 0.2, 1.0);


  /*
  // calc clip_space position of vertex
  vec4 clip_pos = projection * view * model * bt * vec4(Position * 0.2, 1.0);


  // calc clipspace normal
  mat4 vp = view * projection;
  vec3 norm = mat3(transpose(inverse(model * bt))) * vec4(Normal, 1.0).xyz;
  //vec3 norm = mat3(model) * mat3(bt) * Normal;
  vec3 clip_normal = mat3(vp) * norm;

  //clip_pos.xy += (normalize(clip_normal.xy) / vec2(1200, 800)) * 100.0 * clip_pos.w * 2.0;
  gl_Position = clip_pos;
  */


}
