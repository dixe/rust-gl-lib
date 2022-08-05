use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::*;



#[test]
fn text_widget() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let mut text_widget = TextWidget { text: "Hello".to_string() };


    let _ = ui_state.add_widget(Box::new(text_widget), None);

    let screen_contraint = BoxContraint::screen(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);



    assert_eq!(1, ui_state.geom.len());
    for geo in &ui_state.geom {
        assert_eq!(100, geo.size.pixel_w);
        assert_eq!(30, geo.size.pixel_h);
    }

}



#[test]
fn container_widget_1() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let mut container_widget = ContainerWidget { };


    let container_id = ui_state.add_widget(Box::new(container_widget), None);

    let mut text_widget = TextWidget { text: "Hello".to_string() };

    let _ = ui_state.add_widget(Box::new(text_widget), Some(container_id));



    let screen_contraint = BoxContraint::screen(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);



    for geo in &ui_state.geom {
        println!("{:#?}", geo);
    }



    assert_eq!(2, ui_state.geom.len());

    assert_eq!(100, ui_state.geom[0].size.pixel_w);
    assert_eq!(30, ui_state.geom[0].size.pixel_h);

}


#[test]
fn container_widget_2() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let mut container_widget = ContainerWidget { };


    let container_id = ui_state.add_widget(Box::new(container_widget), None);

    let mut text_widget = TextWidget { text: "Hello".to_string() };

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(container_id));
    let _ = ui_state.add_widget(Box::new(text_widget), Some(container_id));



    let screen_contraint = BoxContraint::screen(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);



    for geo in &ui_state.geom {
        println!("{:#?}", geo);
    }


    assert_eq!(3, ui_state.geom.len());

    assert_eq!(200, ui_state.geom[0].size.pixel_w);
    assert_eq!(60, ui_state.geom[0].size.pixel_h);


    assert!(false);
}
