use gl_lib::{gl, objects::cube, shader::{self, Shader}, camera};
use failure;
use gl_lib::{helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::typedef::*;

fn main() -> Result<(), failure::Error> {
    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();
    let viewport = sdl_setup.viewport;
    let gl = sdl_setup.gl.clone();

    let mut camera = camera::Camera::new(viewport.w as f32, viewport.h as f32);
    let mut shader = shader::hitbox_shader::HitboxShader::new(gl).unwrap();
    let cube = cube::Cube::new(gl);

    loop {
        ui.start_frame(&mut sdl_setup.event_pump);

        if ui.button("Reload") {
            shader::reload_object_shader("hitbox", &gl, &mut shader.shader);
        }

        ui.slider(&mut camera.pos.x, -10.0, 10.0);
        ui.slider(&mut camera.pos.y, -10.0, 10.0);
        ui.slider(&mut camera.pos.z, -10.0, 10.0);
        camera.move_to(V3::new(0.0, 10.0, 0.1));

        camera.look_at(V3::new(0.0, 0.0, 0.0));

        let uniforms = shader::hitbox_shader::Uniforms {
            projection: camera.projection(),
            view: camera.view(),
            model: Mat4::identity()
        };

        shader.shader.set_used();
        shader.set_uniforms(uniforms);

        cube.render(gl);
        ui.end_frame();
    }
}
