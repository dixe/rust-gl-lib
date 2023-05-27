use super::*;

#[derive(Default)]
pub struct Options {
    pub show_idx: bool,
    pub show_pos: bool,
    pub v_color: Color,
}

pub fn options_ui(ui: &mut Ui, options: &mut Options) {

    ui.window_begin("Options");

    ui.label("show_idx");
    ui.checkbox(&mut options.show_idx);

    ui.newline();
    ui.label("show_pos");
    ui.checkbox(&mut options.show_pos);

    ui.newline();
    ui.label("v color");
    ui.newline();
    ui.color_picker(&mut options.v_color);

    ui.window_end("Options");

}
