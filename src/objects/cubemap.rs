use std::path::{Path, PathBuf};
use crate::buffer;
use crate::{texture, gl};
use crate::objects::mesh::Mesh;
use image;
use image::imageops;


pub struct Cubemap {
    pub mesh: Mesh,
    pub texture_id: texture::TextureId
}


impl Cubemap {

    pub fn new<P: AsRef<Path> + std::fmt::Debug>(gl: &gl::Gl, path: &P) -> Cubemap {

        let vertices : Vec::<f32> = vec![
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

            1.0, -1.0, -1.0,
            1.0, -1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0, -1.0,
            1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0
        ];


        let indices: Vec<u32> =  (0..(36 as u32)).collect();

        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 3;
        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            // 3
            ebo.bind();
            ebo.static_draw_data(&indices);

            // 4.
            // vertecies
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);

        }

        vbo.unbind();
        vao.unbind();


        let mesh = Mesh {
            vao,
            vbo,
            ebo,
            elements: indices.len() as i32
        };

        let texture_id = load_cubemap_texture(gl, path);
        Self {
            mesh,
            texture_id
        }

    }

    pub fn render(&self, gl: &gl::Gl) {
        texture::set_texture(gl, self.texture_id);
        unsafe {
            // REQUIRED OTHER WISE WE GET A LOT OF FLICKERING
            gl.DepthFunc(gl::LEQUAL)
        }
        self.mesh.render(gl);
        unsafe {
            gl.DepthFunc(gl::LESS)
        }
    }
}



fn load_cubemap_texture<P: AsRef<Path> + std::fmt::Debug>(gl: &gl::Gl, path: &P) -> texture::TextureId {

    let mut p = PathBuf::new();
    p.push(path);


    let mut imgs = vec![];
    println!("Load imgs");
    // we have z is up and not y is up, so order differs from https://learnopengl.com/Advanced-OpenGL/Cubemaps
    for n in ["left.jpg", "right.jpg", "bottom.jpg", "top.jpg", "back.jpg", "front.jpg"] {
        p.push(n);

        let mut img = image::open(&p).unwrap();

        if n == "top.jpg" || n == "bottom.jpg" {
            img = img.flipv().fliph();
        }

        imgs.push(img.into_rgb8());
        p.pop();
    }

    println!("Gen textures");
    let id = texture::gen_texture_cube_map(gl, &imgs);

    println!("DONE textures");

    id
}
