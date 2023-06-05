fn load_by_name(ui: &mut Ui, path: &std::path::PathBuf) -> gl_lib::animations::sheet_animation::SheetAnimation {

    println!("Load from {:?}", path);
    let anim_json = std::fs::read_to_string(path);
    let sheet_anim : gl_lib::animations::sheet_animation::SheetArrayAnimation = match anim_json {
        Ok(json) => {
            serde_json::from_str(&json).unwrap()
        },
        Err(err) => {
            panic!("Error loading json file \n{:?}", err);
        }
    };

    let size = na::Vector2::new(sheet_anim.meta.size.w as f32, (sheet_anim.meta.size.h /2) as f32);

    let mut base_path = path.clone();

    base_path.pop();
    base_path.push(&sheet_anim.meta.image);

    let img = image::open(&base_path).unwrap().into_rgba8();;

    let aspect = img.height() as f32 / img.width() as f32;
    let texture_id = ui.register_image(&img);

    let mut frames = vec![];


    for frame in &sheet_anim.frames {

        frames.push(gl_lib::general_animation::Frame::<gl_lib::animations::sheet_animation::Sprite> {
            data: gl_lib::animations::sheet_animation::Sprite
            {
                x: frame.frame.x,
                y: frame.frame.y,
                w: frame.frame.w,
                h: frame.frame.h,
            },
            frame_seconds: frame.duration as f32 / 1000.0

        });
    }

    let anim = gl_lib::animations::sheet_animation::SheetAnimation {
        texture_id,
        size: na::Vector2::new(sheet_anim.meta.size.w as f32, sheet_anim.meta.size.h as f32),
        animation: gl_lib::general_animation::Animation { frames },
    };

    anim
}
}
