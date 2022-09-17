use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub fn on_input<T>(event: Event, mapping: &ControllerMapping<T>, state: &mut T) {

   match event {
            Event::KeyDown{keycode: Some(kc), .. } => {
                if let Some(f) = mapping.on_press.get(&kc) {
                    f(state);
                }
            }
       _ => {}
   }
}


pub type OnPressFunc<T> = fn(&mut T);

// mapping from input event to fn on T
pub struct ControllerMapping<T> {
    on_press: HashMap<Keycode, OnPressFunc<T>>,
}

impl<T> ControllerMapping<T> {

    pub fn new() -> Self {
        Self {
            on_press: HashMap::new()
        }
    }


    pub fn add_on_press(&mut self, kc: Keycode, f: OnPressFunc<T>) {
        self.on_press.insert(kc, f);
    }

    pub fn exit(&mut self, kc: Keycode) {
        self.add_on_press(kc, exit);
    }

}

pub fn exit<T>(_: &mut T) {
    std::process::exit(0);
}
