use crate::na;
use crate::gl;
use crate::objects::mesh::Mesh;
use crate::animations::skeleton::{load_skins, SkinId, Skins};
use std::collections::HashMap;
use image::{self, buffer::ConvertBuffer};


pub struct GltfData {
    pub meshes: GltfMeshes,
    pub skins: Skins,
    pub animations: HashMap::<SkinId, HashMap<String, Animation>>,
    pub images: Vec::<image::RgbaImage>
}


pub fn meshes_from_gltf(file_path: &str) -> Result<GltfData, failure::Error> {


    let (gltf, buffers, images) = gltf::import(file_path)?;

    let skins = load_skins(&gltf)?;


    let mut inter_joint_index: Vec::<u16> = Vec::new();

    for skin in gltf.skins() {
        for node in skin.joints() {
            let index = node.index();
            inter_joint_index.push(index as u16);
        }
    }

    let mut res = GltfMeshes {
        meshes: std::collections::HashMap::new()
    };

    let mut loaded_images =  vec![];

    for data in &images {
        let w = data.width;
        let h = data.height;

        let img = match data.format {
            gltf::image::Format::R8G8B8A8 => {
                image::RgbaImage::from_raw(w, h, data.pixels.clone()).unwrap()
            },
            gltf::image::Format::R8G8B8 => {
                image::RgbImage::from_raw(w, h, data.pixels.clone()).unwrap().convert()
            },

            _ => {
                panic!("Unsupported image type {:?}", data.format);
            }
        };

        loaded_images.push(img);
    }


    for node in gltf.nodes() {
        match node.mesh() {
            Some(m) => {
                let empty = HashMap::<u16,usize>::new();
                let mesh_name = node.name().unwrap().to_string();
                let index_map = if let Some(skin_id) = skins.mesh_to_skin.get(&mesh_name) {
                    match skins.index_maps.get(&skin_id) {
                        Some(im) => im,
                        None => &empty,
                    }
                }
                else {
                    &empty
                };

                res.meshes.insert(mesh_name, load_gltf_mesh_data(&m, &buffers, &index_map, &inter_joint_index)?);
            },
            _ => {}
        };
    }

    let mut name_to_idx = HashMap::<String, usize>::default();

    let mut i = 0;
    for img in gltf.images() {
        name_to_idx.insert(img.name().unwrap().to_string(), i);
        i += 1;
    }


    println!("{:?}", images.len());
    for tex in gltf.textures() {
        let img = tex.source();
        println!("{:?}", img.name());

        // use name to idx for index into image vector
        match img.source() {
            gltf::image::Source::View { view, mime_type } => {

                println!("Img name {:?}", img.name());
                let buffer = view.buffer();
                match buffer.source() {
                    gltf::buffer::Source::Bin => {
                        println!("View BIN {:?}", (mime_type, buffer.index(), buffer.name()));
                    },
                    gltf::buffer::Source::Uri(s) => {
                        println!("Uri is {:?}", s);
                    }
                }
            },
            gltf::image::Source::Uri { uri, mime_type } => {
                println!("Uri {:?}", (uri, mime_type));
            },

        }

    }




    let mut animations = HashMap::<SkinId, HashMap<String, Animation>>::new();

    for ani in gltf.animations() {
        let name = match ani.name() {
            Some(n) => n.to_string(),
            _ => continue
        };

        println!("\n{:#?}", name);


        let mut frames : Vec::<KeyFrame> = vec![];
        let mut skin_id : Option<SkinId> = None;

        let mut joints_indexes: std::collections::HashMap::<String, usize> = std::collections::HashMap::new();


        let mut base_key_frame = None;

        let mut total_secs = 0.0;

        for channel in ani.channels() {
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));

            let target = &channel.target();
            // set skin_id by first bone.
            if skin_id.is_none() {
                skin_id = skins.node_index_to_skin.get(&target.node().index()).copied();

                if let Some(sid) = skin_id {
                    let skeleton = skins.skeletons.get(&sid).unwrap();
                    for i in 0..skeleton.joints.len() {
                        joints_indexes.insert(skeleton.joints[i].name.clone(), i);
                    }

                    base_key_frame = Some(KeyFrame {
                        start_sec: 0.0,
                        length_sec: 0.0,
                        joints: skeleton.joints.iter().map(|joint| {
                            Transformation {
                                translation: joint.translation,
                                rotation: joint.rotation
                            }
                        }).collect()
                    });
                }
            }


            let joints_index = match joints_indexes.get(target.node().name().unwrap()) {
                Some(i) => *i,
                _ => {
                    println!("Skipping joint {:#?}", target.node().name().unwrap());
                    continue;
                }
            };


            if let Some(inputs) = reader.read_inputs() {

                if frames.len() == 0 {
                    let mut i = 0;
                    let mut last_start = 0.0;
                    for input in inputs {

                        let mut f = base_key_frame.clone().expect("Should have set skin_id and this set base_key_frame from skeleton");
                        f.start_sec = input;
                        frames.push(f);

                        // update frame duration, last frame has duration of 0.0
                        if i > 0 {
                            frames[i-1].length_sec = input - last_start;
                        }

                        last_start = input;
                        i += 1;
                        // total seconds is start of last keyframe, which has length 0 sec
                        total_secs = input;
                    }
                } else {
                    let mut i = 0;
                    for input in inputs {
                        assert_eq!(input, frames[i].start_sec);
                        i += 1;
                    }
                }
            }


            if let Some(read_outputs) = reader.read_outputs() {
                match read_outputs {
                    gltf::animation::util::ReadOutputs::Translations(ts) => {
                        assert_eq!(frames.len(),  ts.len());
                        let mut i = 0;
                        for t in ts {
                            frames[i].joints[joints_index].translation = na::Vector3::new(t[0], t[1], t[2]);
                            i += 1;
                        }
                    },
                    gltf::animation::util::ReadOutputs::Rotations(rs) => {
                        let mut i = 0;
                        for r in rs.into_f32() {

                            let q = na::Quaternion::from(na::Vector4::new(r[0], r[1], r[2], r[3]));

                            frames[i].joints[joints_index].rotation = na::UnitQuaternion::from_quaternion(q);
                            i += 1 ;
                        }
                    },
                    gltf::animation::util::ReadOutputs::Scales(ss) => {
                        for s in ss {
                            let diff = f32::abs(3.0 - (s[0] + s[1] + s[2]));
                            if diff > 0.01 {
                                panic!("Scale was more that 0.01 it might be important\n scale was {}", diff)
                            }
                        }
                    },
                    gltf::animation::util::ReadOutputs::MorphTargetWeights(mtws) => {
                        println!("Got MorphTargetWeights: {:#?}", mtws);
                    }
                }
            }
        }


        if let Some(s_id) = skin_id {
            if !animations.contains_key(&s_id) {
                animations.insert(s_id, Default::default());
            }

            let map : &mut HashMap::<String, Animation> = animations.get_mut(&s_id).unwrap();

            map.insert(name.clone(), Animation {frames, total_secs } ) ;
        }
    }

    println!("Meshes loaded {:#?}", res.meshes.keys());

    Ok(GltfData { meshes:res, skins, animations, images: loaded_images})

}

fn load_gltf_mesh_data(mesh: &gltf::mesh::Mesh, buffers: &Vec<gltf::buffer::Data>, index_map: &std::collections::HashMap<u16,usize>, inter_joint_index: &Vec::<u16>) -> Result<GltfMesh, failure::Error> {

    let name = mesh.name().unwrap().to_string();

    let mut pos_data = Vec::new();

    let mut normal_data = Vec::new();

    let mut smooth_normal_data : Vec::<[f32; 3]> = Vec::new();

    let mut tex_data = Vec::new();

    let mut indices_data = Vec::new();

    let mut joints_data = Vec::new();

    let mut weights_data = Vec::new();

    let set = 0;


    for primitive in mesh.primitives() {

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        if let Some(iter) = reader.read_positions() {
            for pos in iter {
                let p1 = na::Vector3::new(pos[0], pos[1], pos[2]);
                pos_data.push(p1);
            }
        }

        if let Some(iter) = reader.read_normals() {
            for norm in iter {
                //println!("{:?}", norm);
                normal_data.push(norm);
            }
        }

        if let Some(reader) = reader.read_tex_coords(set) {
            for tex in reader.into_f32() {
                tex_data.push(tex);
            }
        }

        if let Some(reader) = reader.read_indices() {
            for tex in reader.into_u32() {
                indices_data.push(tex);
            }
        }


        if let Some(reader) = reader.read_weights(set) {
            for w in  reader.into_f32() {
                weights_data.push(w);
            }
        }


        if let Some(reader) = reader.read_joints(set) {
            let mut c = 0;
            for j in reader.into_u16() {
                let mut data: [usize; 4] = [0; 4];
                for (i, index) in j.iter().enumerate() {
                    // index is into the skins.joints array, which has a list of node indexes
                    // so we have to map from index into joints to
                    data[i] = match index_map.get(&inter_joint_index[*index as usize]) {
                        Some(mapping) => *mapping,
                        None => {
                            println!("c={}, j={:?}\nWeight Data = {:?}", c, j, weights_data[c]);
                            panic!("Non mapped bone has weights. oCheck weight paint for {}", *index)
                        }
                    };
                }

                c += 1;
                joints_data.push(data);
            }
        }
        //panic!("");

    }


    // find all vertceis at same point, sum normals and normalize, to get smoothshadeNormal
    let mut smooth = HashMap::<Key, na::Vector3::<f32>>::default();

    for p in &pos_data {
        let k : Key = p.into();
        if !smooth.contains_key(&k) {
            smooth.insert(k, na::Vector3::<f32>::new(0.0, 0.0, 0.0));
        }
    }

    // summ all normals for each pos
    for i in 0..normal_data.len() {
        let k = pos_data[i].into();

        if let Some(n) = smooth.get_mut(&k) {
            let new : na::Vector3::<f32> = *n  + na::Vector3::new(normal_data[i][0], normal_data[i][1], normal_data[i][2]);
            *n = new;
        }
    }

    // summ all normals for each pos
    for i in 0..normal_data.len() {
        let k = pos_data[i].into();
        let n = smooth.get(&k).unwrap().normalize();
        let sm = [n.x, n.y, n.z];
        smooth_normal_data.push(sm);
    }

    let vertex_weights = reduce_to_2_joints(&joints_data, &weights_data);


    Ok(GltfMesh {
        name,
        pos_data,
        normal_data,
        smooth_normal_data,
        indices_data,
        tex_data,
        vertex_weights,
    })
}


fn reduce_to_2_joints(joints_data: &Vec<[usize; 4]>, weights_data: &Vec<[f32; 4]>) -> Vec<VertexWeights> {
    // find the two largest weights and use them also normalize the two weights sum to 1

    let mut res = Vec::new();

    for j_index in 0..joints_data.len() {
        let mut max1 = 0.0;
        let mut max2 = 0.0;
        let mut max1_i = 0;
        let mut max2_i = 0;
        for w_index in 0..4 {

            let w = weights_data[j_index][w_index];

            if w >= max1 {
                max2 = max1;
                max2_i = max1_i;
                max1 = w;
                max1_i = w_index;
            }
            else if w >= max2 {
                max2 = w;
                max2_i = w_index;
            }
        }

        if max1_i == max2_i {
            max2 = 0.0;
        }

        if max2 == 0.0 {
            max2_i = max1_i;
        }


        let sum = max1 + max2;
        let max1 = max1 / sum;
        let max2 = max2 / sum;

        let joint_1 = joints_data[j_index][max1_i];
        let joint_2 = joints_data[j_index][max2_i];


        res.push(VertexWeights {
            joints: [joint_1, joint_2],
            weights: [ max1, max2]
        });
    }

    res
}



// alternative just load the data. and then we can instanciate it if needed
#[derive(Clone)]
pub struct GltfMesh {
    pub name: String,
    pub pos_data: Vec<na::Vector3::<f32>>,
    pub normal_data: Vec<[f32; 3]>,
    pub smooth_normal_data: Vec<[f32; 3]>,
    pub indices_data: Vec<u32>,
    pub tex_data: Vec<[f32; 2]>,
    pub vertex_weights: Vec<VertexWeights>,
}

impl GltfMesh {
    pub fn triangles(&self) -> Vec::<Triangle> {
        panic!("");
        let mut res = Vec::new();
        for i in (0..self.indices_data.len()).step_by(3) {
            let v0_i = self.indices_data[i];
            let v1_i = self.indices_data[i + 1];
            let v2_i = self.indices_data[i + 2];

            let v0 = self.pos_data[v0_i as usize];
            let v1 = self.pos_data[v1_i as usize];
            let v2 = self.pos_data[v2_i as usize];

            let n0 = self.normal_data[v0_i as usize];
            let _n1 = self.normal_data[v1_i as usize];
            let n2 = self.normal_data[v2_i as usize];
            let normal = ((na::Vector3::new(n0[0], n0[1], n0[2]) + na::Vector3::new(n0[0], n0[1], n0[2]) + na::Vector3::new(n2[0], n2[1], n2[2])) / 3.0).normalize();


            // use v0 to find d
            let d = -(normal.x * v0.x + normal.y * v0.y + normal.z * v0.z);
            let t = Triangle { v0, v1, v2, normal, d};

            res.push(t);
        }

        res
    }

    pub fn get_mesh(&self, gl: &gl::Gl) -> Mesh {

        let mut mesh = Mesh::empty(gl);


        let skin_data = if self.vertex_weights.len() > 0 {
            Some(&self.vertex_weights)
        }
        else {
            None
        };

        bind_mesh_data(
            gl,
            &self.pos_data,
            &self.normal_data,
            &self.smooth_normal_data,
            &self.indices_data,
            &self.tex_data,
            &mut mesh,
            skin_data
        );

        mesh
    }
}


#[derive(PartialEq, Clone, Eq, Hash, Copy)]
struct Key {
    x: u32,
    y: u32,
    z: u32
}

impl std::convert::From<&na::Vector3::<f32>> for Key {
    fn from(v : &na::Vector3::<f32>) -> Self {
        Self {
            x: v.x.to_bits(),
            y: v.y.to_bits(),
            z: v.z.to_bits(),
        }
    }
}


impl std::convert::From<na::Vector3::<f32>> for Key {
    fn from(v : na::Vector3::<f32>) -> Self {
        Self {
            x: v.x.to_bits(),
            y: v.y.to_bits(),
            z: v.z.to_bits(),
        }
    }
}


pub struct GltfMeshes {
    pub meshes: std::collections::HashMap::<String, GltfMesh>
}


impl GltfMeshes {

    pub fn hitboxes(&self, base_name: &str) -> Vec::<(String, Vec<na::Vector3::<f32>>)>{

        //println!("HITBOXES FOR {}", base_name);
        let mut res = Vec::new();
        for mesh_data in self.meshes.iter().filter(|kv| kv.0.starts_with(base_name) && kv.0.contains("hitbox")).map(|kv| kv.1) {
            // meshes are triangulated, so we want to detriangulate them before we pass them on.
            // They should have 8 vertices
            // from looking at data it seems liek for at box the layout is v0,v0,v0, v1, v1,v1, v2,v2,v2, v3,v3,v3... v7,v7,v7,
            // So we can just loop over in with step of 2 and thus get the vertices

            let mut hitbox = Vec::new();
            let mut final_hitbox = Vec::new();
            for i in (0..24).step_by(3) {
                hitbox.push(mesh_data.pos_data[i]);
            }

            final_hitbox.push(hitbox[0]);
            final_hitbox.push(hitbox[4]);
            final_hitbox.push(hitbox[6]);
            final_hitbox.push(hitbox[2]);
            final_hitbox.push(hitbox[1]);
            final_hitbox.push(hitbox[5]);
            final_hitbox.push(hitbox[7]);
            final_hitbox.push(hitbox[3]);


            res.push((mesh_data.name.clone(), final_hitbox));

        }

        res
    }

    pub fn triangles(&self, name: &str) -> Vec::<Triangle> {
        let mut res = Vec::new();
        for mesh_data in self.meshes.iter().filter(|kv| kv.0 == name).map(|kv| kv.1) {
            // meshes are triangulated

            res = mesh_data.triangles();
        }

        res
    }


    pub fn get_mesh(&self, gl: &gl::Gl, name: &str) -> Option<Mesh> {

        if let Some(m) = self.meshes.get(name) {
            return Some(m.get_mesh(gl));
        }
        return None
    }

}



fn bind_mesh_data(
    gl: &gl::Gl,
    pos_data: &Vec<na::Vector3::<f32>>,
    norm_data: &Vec<[f32; 3]>,
    smooth_norm_data: &Vec<[f32; 3]>,
    ebo_data: &Vec<u32>,
    tex_data: &Vec<[f32; 2]>,
    mesh: &mut Mesh,
    skinning_data: Option<&Vec<VertexWeights>>,

) {

    let mut vertex_data = Vec::<f32>::new();


    let indices_count = ebo_data.len();



    for i in 0..pos_data.len() {
        vertex_data.push(pos_data[i].x);
        vertex_data.push(pos_data[i].y);
        vertex_data.push(pos_data[i].z);

        //NORMAL

        vertex_data.push(norm_data[i][0]);
        vertex_data.push(norm_data[i][1]);
        vertex_data.push(norm_data[i][2]);

        match skinning_data {

            Some(s_data) => {
                // BONE WEIGHTS
                vertex_data.push(s_data[i].weights[0]);
                vertex_data.push(s_data[i].weights[1]);

                // BONE INDICES

                vertex_data.push(s_data[i].joints[0] as f32);
                vertex_data.push(s_data[i].joints[1] as f32);
            },
            _ => {
                // BONE WEIGHTS
                vertex_data.push(0.0);
                vertex_data.push(0.0);

                // BONE INDICES

                vertex_data.push(-1.0);
                vertex_data.push(-1.0);

            }
        }
        // TEXTURE INFO
        vertex_data.push(tex_data[i][0]);
        vertex_data.push(tex_data[i][1]);

        // SMOOTH NORMALS, FX FOR STENCIL OUTLINE
        vertex_data.push(smooth_norm_data[i][0]);
        vertex_data.push(smooth_norm_data[i][1]);
        vertex_data.push(smooth_norm_data[i][2]);
    }



    let stride = ((3 + 3 + 2 + 2 + 2 + 3) * std::mem::size_of::<f32>()) as gl::types::GLint;
    unsafe {
        // 1
        mesh.vao.bind();

        // 2.
        mesh.vbo.bind();
        mesh.vbo.static_draw_data(&vertex_data);

        //3
        mesh.ebo.bind();
        gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (ebo_data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            ebo_data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // 4.
        // vertecies
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            0 as *const gl::types::GLvoid,
        );
        gl.EnableVertexAttribArray(0);

        // normals
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(1);


        // bone weights

        gl.VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(2);

        // bone indices
        gl.VertexAttribPointer(
            3,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (8 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl.EnableVertexAttribArray(3);

        // bone indices
        gl.VertexAttribPointer(
            4,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (10 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(4);

        // normals
        gl.VertexAttribPointer(
            5,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (12 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(5);


    }

    mesh.vbo.unbind();
    mesh.vao.unbind();
    mesh.elements = indices_count as i32;
}





#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub v0: na::Vector3::<f32>,
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>,
    pub normal: na::Vector3::<f32>,
    pub d : f32,
}

#[derive(Debug, Copy, Clone)]
pub struct VertexWeights {
    // maybe keep the actual vertex index instead of having it just as the index in the vec this is stored in
    joints: [usize; 2],
    weights: [f32; 2],
}


#[derive(Debug, Clone)]
pub struct Animation {
    pub total_secs: f32,
    pub frames: Vec::<KeyFrame>
}

#[derive(Debug, Default, Clone)]
pub struct KeyFrame {
    pub start_sec: f32,
    pub length_sec: f32,
    pub joints: Vec<Transformation>,
}

impl KeyFrame {
    pub fn interpolate(&self, other: &KeyFrame, t: f32, output: &mut KeyFrame) {
        for i in 0..self.joints.len() {
            output.joints[i].translation = self.joints[i].translation.lerp(&other.joints[i].translation, t);
            output.joints[i].rotation = self.joints[i].rotation.slerp(&other.joints[i].rotation, t);

        }
    }

    pub fn end_time(&self) -> f32 {
        self.start_sec + self.length_sec
    }
}

#[derive(Debug, Copy, Default, Clone)]
pub struct Transformation {
    pub translation: na::Vector3::<f32>,
    pub rotation: na::UnitQuaternion::<f32>,
}
