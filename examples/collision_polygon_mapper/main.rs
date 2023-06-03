use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::{*};
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::widgets::PolygonOptions;



use gl_lib::collision2d::polygon::{Polygon};

use gl_lib::texture::TextureId;



type V2 = na::Vector2::<f32>;
type V2i = na::Vector2::<i32>;
type V3 = na::Vector3::<f32>;


fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    ui.drawer2D.font_cache.fonts_path = Some("assets/fonts/".to_string());

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let mut edit1 = load(&mut ui);

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui);

        if ui.button("Save") {
            match serde_json::to_string(&edit1.polygon) {
                Ok(json) => {
                    std::fs::write("examples/collision_polygon_mapper/Sword.json", json);
                },
                Err(err) => {
                    println!("Fail to save\n{:?}", err);
                }
            }
        }

        img_edit(&mut ui, &mut edit1);

        window.gl_swap_window();

    }
}



fn load(ui: &mut Ui) -> ImageEdit {

    // TODO: Load from path, maybe make this changeable
    let img = image::open("examples/top_rpg/assets/Sword.png").unwrap().into_rgba8();

    let texture_id = ui.register_image_nearest(&img);


    let base_size = V2::new(img.width() as f32, img.height() as f32);

    let vertices_json = std::fs::read_to_string("examples/collision_polygon_mapper/Sword.json");


    let polygon = match vertices_json {
        Ok(json) => {
            serde_json::from_str(&json).unwrap()
        },
        Err(err) => {
            println!("Error loading json file, creating default polygon \n{:?}", err);
            Polygon {
                vertices: vec![V2::new(32.0, 0.0),
                               V2::new(0.0, 32.0),
                               V2::new(32.0, 32.0),
                ]
            }
        }
    };

    let mut edit = ImageEdit {
        polygon,
        texture_id,
        base_size,
        anchor: na::Vector2::new(200, 100),
        name: "Target".to_string(),
        options: Default::default()
    };

    edit.options.transform.scale = 10.0;

    edit
}


fn img_edit(ui: &mut Ui, edit: &mut ImageEdit) {

    ui.label("Scale");
    ui.slider(&mut edit.options.transform.scale, 1.0, 20.0);

    let scale = edit.options.transform.scale;

    ui.image_at(edit.texture_id, edit.base_size * scale, edit.anchor.x, edit.anchor.y);
    ui.drag_point(&mut edit.anchor, 10.0);

    edit.options.transform.translation.x = edit.anchor.x as f32;
    edit.options.transform.translation.y = edit.anchor.y as f32;


    let mut output_offset = edit.anchor;
    output_offset.x -= 40;
    output_offset.y += 30;

    // draw "center" of polygon on base image with anchor = output_offset
    ui.image_at(edit.texture_id, edit.base_size, output_offset.x, output_offset.y);

    ui.polygon_editor(&mut edit.polygon, &mut edit.options);

    // draw ""Correct" polygon, with translation offset
    let mut transform = edit.options.transform;

    transform.translation = output_offset.to_v2();
    transform.scale = 1.0;
    ui.view_polygon(&mut edit.polygon, &transform);

}

trait ToV2 {
    fn to_v2(&self) -> V2;
}

impl ToV2 for V2i {
    fn to_v2(&self) -> V2 {
        V2::new(self.x as f32, self.y as f32)
    }

}

struct ImageEdit {
    name: String,
    polygon: Polygon,
    texture_id: TextureId,
    base_size: V2,
    anchor: V2i,
    options: PolygonOptions
}

fn screen_to_img_coords(mut v: V2, anchor: V2i, scale: f32) -> V2{
    v.x -= anchor.x as f32;
    v.y -= anchor.y as f32;

    v *= 1.0/scale;
    v
}

fn handle_inputs(ui: &mut Ui) {





    for e in &ui.frame_events {
        match e {
            _ => {}

        }
    }
}
