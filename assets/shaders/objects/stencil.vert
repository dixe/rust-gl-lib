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
layout (location = 5) in vec3 SmoothNormal;

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

  //gl_Position = projection * view * model * bt * vec4(Position + Normal * 0.2, 1.0);


  // Complex version where we transform to clip space, so we can give a uniform length
  // calc clip_space position of vertex

  vec4 clip_pos = projection * view * model * bt * vec4(Position, 1.0);

  // calc clipspace normal
  vec3 clip_normal = mat3(projection * view) * mat3(transpose(inverse(model * bt))) * SmoothNormal;


  float ndc_width = 0.025;
  // not really aspect, we we just set it to vec2(1200, 800); Then we always draw pixel_width wide outline.
  // But it can look a little funny when obects are far aways and outline is thicker then the object is self
  // currently it gives an outline that scales with depth, so when far away outline is thiner

  vec2 aspect = vec2(1920.0/1080.0, 1.0) * clip_pos.w * 1.0/ndc_width;
  //vec2 aspect = vec2(1200, 800) * 1.0/pixel_width;
  clip_pos.xy += normalize(clip_normal.xy) / aspect * 2.0 * clip_pos.w;

  gl_Position = clip_pos;
}
