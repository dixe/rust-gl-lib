use gl_lib::{gl, na, helpers};
use gl_lib::particle_system::*;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use deltatime;
use gl_lib::text_rendering::font::{Font, MsdfFont, FntFont};
use gl_lib::shader::BaseShader;


fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let mut emitter = emitter::Emitter::new(1000, emitter::emit_1, emitter::update_1);
    let mut delta_time = deltatime::Deltatime::new();

    let mut amount = 1;

    let mut fonts = generate_font_vec(gl).unwrap();

    let shader_names = ["msdf_text_render", "softmask_text_render", "sdf_text_render", "alpha_mask_text_render"];

    let mut current_shader = "softmask_text_render".to_string();

    let mut current_font = fonts[0].name().to_string();
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        delta_time.update();

        ui.consume_events(&mut event_pump);

        if ui.button("Emit super") {
            for _ in 0..amount {
                emitter.emit();
            }
        }

        ui.label("Amount");
        ui.slider( &mut amount, 1, 100);


        if ui.button("Reload font list") {
            fonts = generate_font_vec(gl).unwrap();
        }
        ui.newline();
        let mut pixel_size = ui.style.text_styles.small.pixel_size;

        if ui.button("-") {
            pixel_size -= 1;
        }
        ui.slider(&mut pixel_size, 6, 60);
        if ui.button("+") {
            pixel_size += 1;
        }

        ui.label(&format!("{pixel_size}"));

        ui.style.text_styles.small.pixel_size = pixel_size;

        ui.newline();

        if let Some(name) = choose_font(&mut ui, &fonts) {
            current_font = name.to_string();
        }

        ui.newline();



/*
        if ui.button("Change") {
            index = (index + 1) % 2;
            update_font(&mut ui, &ts[index]);
            update_shaders(&mut ui, gl, &ts[index]);
        }

        if ui.button("Reload") {
            update_font(&mut ui, &ts[index]);
            update_shaders(&mut ui, gl, &ts[index]);
        }
*/

        let dt = delta_time.time();
        emitter.update(dt);
        emitter.draw_all(&ui.drawer2D);

        ui.newline();
        ui.newline();
        ui.newline();
        ui.label(&current_font);
        ui.newline();
        ui.label(&current_shader);
        ui.newline();
        if let Some(name) = choose_shader(&mut ui, gl, &shader_names, &mut fonts) {
            current_shader = name;
        }


        ui.newline();
        let text = "This is a test text\nWith numbers 1,2,3 456 and float 3.22, and 0.32\n Capital small and LetTerS ofDiferRenSiz With  Spa   a   a  ce  e  e s and not.\n\nChaing size will show how the renderere works /**-+.";

        ui.small_text(text);

        window.gl_swap_window();
    }
}


fn choose_shader(ui: &mut Ui, gl: &gl::Gl, shader_names: &[&str], fonts: &mut Vec::<Font>) -> Option<String> {
    let mut r = None;
    for name in shader_names {
        if ui.button(&format!("{name}")) {
            update_shaders(ui, gl, name, fonts);
            r = Some(name.to_string());
        }
    }
    r
}

fn choose_font(ui: &mut Ui, fonts: &[Font]) -> Option<String> {
    let mut r = None;
    for font in fonts {
        if ui.button(&format!("{}-{}", font.name(), font.size())) {
            ui.drawer2D.font_cache.default = font.clone();
            r = Some(font.name().to_string());
        }
    }
    r
}


fn generate_font_vec(gl: &gl::Gl) -> std::io::Result<Vec::<Font>> {

    let mut res = vec![];

    for dir_entry in std::fs::read_dir("assets/fonts/")? {
        let dir = dir_entry?;
        let dp = dir.path();
        let file = dp.to_str().unwrap();

        if file.ends_with(".fnt") {
            let inner_font = FntFont::load_fnt_font(file).unwrap();
            res.push(Font::fnt(gl, inner_font));
        }

        if file.ends_with(".json") {
            let inner_font = MsdfFont::load_from_paths(&file,  &file.replace(".json", ".png")).unwrap();
            res.push(Font::msdf(gl, inner_font));

        }
    }

    res.sort_by(|a, b| a.name().partial_cmp(&b.name()).unwrap());
    Ok(res)
}

/*
fn update_font(ui: &mut Ui, name: &str) {


    let inner_font = MsdfFont::load_from_paths(&format!("assets/fonts/{name}.json"), &format!("assets/fonts/{t}_consolas.png")).unwrap();
    let font = Font::Msdf(inner_font);
    ui.drawer2D.tr.change_font(&ui.drawer2D.gl, font);
}
 */


fn update_shaders(ui: &mut Ui, gl: &gl::Gl, name: &str, fonts: &mut Vec::<Font>) {


    match BaseShader::new(gl, &std::fs::read_to_string(format!("assets/shaders/{name}.vert")).unwrap(), &std::fs::read_to_string(format!("assets/shaders/{name}.frag")).unwrap()) {
        Ok(shader) => {
            for font in fonts {
                font.change_shader(shader.clone());
            }
            ui.drawer2D.tr.font_mut().change_shader(shader.clone());
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
