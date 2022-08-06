use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::*;



#[test]
fn text_widget() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let text_widget = TextWidget { text: "Hello".to_string() };


    let _ = ui_state.add_widget(Box::new(text_widget), None, None);

    let screen_contraint = BoxContraint::new(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);



    assert_eq!(1, ui_state.geom.len());
    for geo in &ui_state.geom {
        assert_eq!(100, geo.size.pixel_w);
        assert_eq!(30, geo.size.pixel_h);
    }

}



#[test]
fn row_widget_1() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let row_widget = RowWidget { };


    let row_id = ui_state.add_widget(Box::new(row_widget), None, None);

    let text_widget = TextWidget { text: "Hello".to_string() };

    let _ = ui_state.add_widget(Box::new(text_widget), Some(row_id), None);



    let screen_contraint = BoxContraint::new(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);



    assert_eq!(2, ui_state.geom.len());

    assert_eq!(100, ui_state.geom[0].size.pixel_w);
    assert_eq!(30, ui_state.geom[0].size.pixel_h);
}


#[test]
fn row_widget_2() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let row_widget = RowWidget { };


    let row_id = ui_state.add_widget(Box::new(row_widget), None, None);

    let text_widget = TextWidget { text: "Hello".to_string() };

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(row_id), None);
    let _ = ui_state.add_widget(Box::new(text_widget), Some(row_id), None);



    let screen_contraint = BoxContraint::new(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);



    assert_eq!(3, ui_state.geom.len());

    assert_eq!(200, ui_state.geom[0].size.pixel_w);
    assert_eq!(30, ui_state.geom[0].size.pixel_h);

    assert_eq!(100, ui_state.geom[2].pos.x);

}



#[test]
fn row_widget_3() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let row_widget = RowWidget { };


    let row_id = ui_state.add_widget(Box::new(row_widget), None, None);

    let text_widget = TextWidget { text: "Hello".to_string() };

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(row_id), None);
    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(row_id), None);
    let _ = ui_state.add_widget(Box::new(text_widget), Some(row_id), None);



    let screen_contraint = BoxContraint::new(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);



    assert_eq!(4, ui_state.geom.len());

    assert_eq!(300, ui_state.geom[0].size.pixel_w);
    assert_eq!(30, ui_state.geom[0].size.pixel_h);

    assert_eq!(200, ui_state.geom[3].pos.x);

}


#[test]
fn row_of_columns_widget_1() {

    let width = 1000;
    let height = 600;

    let mut ui_state = UiState::new();

    let row_widget = RowWidget { };

    let row_id = ui_state.add_widget(Box::new(row_widget), None, None);


    let column = ColumnWidget { };

    let text_widget = TextWidget { text: "Hello".to_string() };


    // Setup column 1 with 2 texts
    let c1_id = ui_state.add_widget(Box::new(column.clone()), Some(row_id), None);

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(c1_id), None);
    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(c1_id), None);


    // Setup column 2 with 3 texts

    let c2_id = ui_state.add_widget(Box::new(column.clone()), Some(row_id), None);

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(c2_id), None);
    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(c2_id), None);
    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(c2_id), None);


    let screen_contraint = BoxContraint::new(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);


    assert_eq!(8, ui_state.geom.len());

    assert_eq!(200, ui_state.geom[0].size.pixel_w);
    assert_eq!(90, ui_state.geom[0].size.pixel_h);
}




#[test]
fn widget_flex_1() {

    let width = 300;
    let height = 600;

    let mut ui_state = UiState::new();

    let row_widget = RowWidget { };


    let row_id = ui_state.add_widget(Box::new(row_widget), None, None);

    let text_widget = TextWidget { text: "Hello".to_string() };

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(row_id), Some(WidgetConstraint::flex_width(2)));

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(row_id), Some(WidgetConstraint::flex_width(1)));



    let screen_contraint = BoxContraint::new(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);

    println!("{:#?}", ui_state.geom);


    assert_eq!(3, ui_state.geom.len());
    assert_eq!(30, ui_state.geom[0].size.pixel_h);

    assert_eq!(200, ui_state.geom[1].size.pixel_w);
    assert_eq!(100, ui_state.geom[2].size.pixel_w);

}



#[test]
fn widget_flex_2() {

    let width = 300;
    let height = 600;

    let mut ui_state = UiState::new();

    let column_widget = ColumnWidget { };


    let column_id = ui_state.add_widget(Box::new(column_widget), None, None);

    let text_widget = TextWidget { text: "Hello".to_string() };

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(column_id), Some(WidgetConstraint::flex_height(2)));

    let _ = ui_state.add_widget(Box::new(text_widget.clone()), Some(column_id), Some(WidgetConstraint::flex_height(1)));



    let screen_contraint = BoxContraint::new(width, height);
    layout_widgets(&screen_contraint, &mut ui_state);

    println!("{:#?}", ui_state.geom);


    assert_eq!(3, ui_state.geom.len());
    assert_eq!(100, ui_state.geom[0].size.pixel_w);
    assert_eq!(600, ui_state.geom[0].size.pixel_h);

    assert_eq!(400, ui_state.geom[1].size.pixel_h);
    assert_eq!(200, ui_state.geom[2].size.pixel_h);

}
