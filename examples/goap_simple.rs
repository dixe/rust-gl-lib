use gl_lib::goap::*;
use gl_lib::{gl, helpers, na};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use std::rc::Rc;


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let gl = &sdl_setup.gl;
    let drawer_2d = Drawer2D::new(&gl, sdl_setup.viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    let mut event_pump =  sdl_setup.sdl.event_pump().unwrap();

    let buy = Action {
        name: "BuyAxe".into(),
        cost: 2,
        pre: vec![
            Cond { name: "AtShop".into(), state: true},
        ],
        post: vec![
            Cond { name: "HasAxe".into(), state: true},
        ]
    };

    let loot_axe = Action {
        name: "LootAxe".into(),
        cost: 1,
        pre: vec![
            Cond { name: "HasCorpse".into(), state: true},
        ],
        post: vec![
            Cond { name: "HasAxe".into(), state: true},
        ]
    };

    let to_shop = Action {
        name: "GoToShop".into(),
        cost: 1,
        pre: vec![
            Cond { name: "HasMoney".into(), state: true},
        ],
        post:  vec![
            Cond { name: "AtShop".into(), state: true},
        ]
    };


    let get_money = Action {
        name: "GetMoney".into(),
        cost: 1,
        pre: vec![],
        post:  vec![
            Cond { name: "HasMoney".into(), state: true},
        ].into()
    };



    let axe_goal = Goal {
        name: "GetAxe".into(),
        desired_state: vec![Cond { name: "HasAxe".into(), state: true}],
        is_valid: axe_goal_valid
    };


    let money_goal = Goal {
        name: "GetMoney".into(),
        desired_state: vec![],
        is_valid: money_goal_valid
    };


    let chill_goal = Goal {
        name: "Chill".into(),
        desired_state: vec![],
        is_valid: |_| true
    };


    let mut world_state = State::default();


    let goals = [axe_goal, money_goal, chill_goal];

    let actions = [buy, to_shop, get_money, loot_axe];

    let mut rm: Vec::<Rc::<str>> = vec![];
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        if ui.button("Plan") {
            if let Some((goal, p)) = plan(&goals, &actions, &mut world_state) {
                println!("Found plan {:?} -- {:?}", goal.name, p);
                // update state as if plan has excuted
                for name in &p {
                    for a in &actions {
                        if name == &a.name {
                            for post in &a.post {
                                world_state.insert(post.name.clone(), true);
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

fn money_goal_valid(state: &State) -> bool {

    if let Some(val) = state.get("HasMoney".into()) {
        // HasMoney is in worldState, if true, goal not valid, since we want more
        return !val;
    }

    // HasMoney is not in world state, and we want some so goal is valid
    true
}


fn axe_goal_valid(state: &State) -> bool {

    if let Some(val) = state.get("HasAxe".into()) {
        // HasAxe is in worldState, if true, goal not valid, since we only want one
        // if false, goal is valid since we want an axe
        return !val;
    }

    // HasAxe is not in world state, and we want one so goal is valid
    true
}
