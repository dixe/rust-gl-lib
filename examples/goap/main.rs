use gl_lib::goap::*;
use gl_lib::{gl, helpers, na};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use std::rc::Rc;
use std::fs;
use toml;
use itertools::Itertools;
use std::collections::HashMap;


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let gl = &sdl_setup.gl;
    let drawer_2d = Drawer2D::new(&gl, sdl_setup.viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    let mut event_pump = sdl_setup.sdl.event_pump().unwrap();


    let axe_goal = Goal {
        name: "GetAxe".into(),
        desired_state: HashMap::from([("HasAxe".into(),true)]),
        is_valid: HashMap::from([("HasAxe".into(), false)]),
    };



    let chill_goal = Goal {
        name: "Chill".into(),
        desired_state: HashMap::from([]),
        is_valid: HashMap::from([]),
    };


    let mut world_state = State::default();


    let goals = [axe_goal, chill_goal];

    let actions_str = fs::read_to_string("examples/goap.toml").unwrap();
    let mut actions : Actions = toml::from_str(&actions_str).unwrap();

    let mut rm: Vec::<Rc::<str>> = vec![];
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        if ui.button("Plan") {
            if let Some((goal, p)) = plan(&goals, &actions, &world_state) {
                println!("Found plan {:?} -- {:?}", goal.name, p);
                // update state as if plan has excuted
                for name in &p {
                    for a in &actions.action {
                        if name == &a.name {
                            for (post, _) in &a.post {
                                world_state.insert(post.clone(), true);
                            }
                            break;
                        }
                    }
                }
            }
        }


        if ui.button("Print world") {
            println!("{:?}", world_state);
        }

        if ui.button("Reload_actions") {
            let actions_str = fs::read_to_string("examples/goap.toml").unwrap();
            match toml::from_str::<Actions>(&actions_str) {
                Ok(a) => {
                    actions = a;
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }


        if ui.button("Print actions") {
            println!("{:#?}", &actions);
        }


        ui.newline();
        for (name, val) in &world_state {
            if ui.button(&format!("Remove:{}", &name)) {
                rm.push(name.clone());
            }
            ui.newline();
        }

        for name in &rm {
            world_state.remove(name);
        }
        rm.clear();

        sdl_setup.window.gl_swap_window();
    }
}
