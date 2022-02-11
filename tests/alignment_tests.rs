use gl_lib::sdl_gui::layout::{Length::*};
use gl_lib::sdl_gui::{
    window,
    layout::*,
};


#[derive(Debug, Clone, Copy)]
enum Message {
    Add
}



#[test]
fn align_left_engine() {

    let width = 1000;
    let height = 600;

    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height).unwrap();

    let btn_width = 20;
    let spacing = 10;

    let col = Column::new()
        .width(Fill)
        .height(FitContent)
        .padding(10.0)
        .spacing(spacing as f32)
        .add(Row::new()
             .width(Fill)
             .add_attribute(Attribute::Spacing(10.0))
             .add(Button::new("Right", Some(Message::Add))
                  .width(Px(btn_width as u32))
                  .align_left()
                  .height(Fill)));


    let aligned_elements = engine::align_tree(col.into(), engine::Size { w: width as f32, h: height as f32 }, window.text_renderer());

    let mut found = false;
    for elm in &aligned_elements {
        println!("{}.size = {:?}", elm.node.name(), elm.realized_size);
    }

    for elm in &aligned_elements {
        if elm.node.name() == "button" {
            assert_eq!(elm.realized_size.x as i32, spacing);
            assert_eq!(elm.realized_size.y as i32, spacing);
        }
        found = true;
    }


    assert!(found);
}


#[test]
fn align_right_engine() {

    let width = 1000;
    let height = 600;

    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height).unwrap();

    let btn_width = 20;
    let spacing = 10;
    let padding = 10;

    let col = Column::new()
        .width(Fill)
        .height(FitContent)
        .padding(10.0)
        .spacing(spacing as f32)
        .add(Row::new()
             .width(Fill)
             .add_attribute(Attribute::Spacing(10.0))
             .add(Button::new( "Right", Some(Message::Add))
                  .width(Px(btn_width as u32))
                  .align_right()
                  .height(Fill)));




    let aligned_elements = engine::align_tree(col.into(), engine::Size { w: width as f32, h: height as f32 }, window.text_renderer());

    for elm in &aligned_elements {
        println!("{}.size = {:?}", elm.node.name(), elm.realized_size);
    }

    let mut found = false;
    for elm in &aligned_elements {
        if elm.node.name() == "button" {
            assert_eq!(elm.realized_size.x as i32, (width as i32 - btn_width - padding));
            assert_eq!(elm.realized_size.y as i32, spacing);
        }
        found = true;
    }

    assert!(found);

}


#[test]
fn align_center_engine() {

    let width = 1000;
    let height = 600;

    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height).unwrap();

    let btn_width = 20;
    let spacing = 10;

    let col = Column::new()
        .width(Fill)
        .height(FitContent)
        .padding(spacing as f32)
        .spacing(spacing as f32)
        .add(Row::new()
             .width(Fill)
             .add_attribute(Attribute::Spacing(spacing as f32))
             .add(Button::new( "Right", Some(Message::Add))
                  .width(Px(btn_width as u32))
                  .align_center()
                  .height(Fill)));

    let aligned_elements = engine::align_tree(col.into(), engine::Size { w: width as f32, h: height as f32 }, window.text_renderer());

    for elm in &aligned_elements {
        println!("{}.align = {:?}", elm.node.name(), elm.realized_size);
    }

    let mut found = false;
    for elm in &aligned_elements {
        if elm.node.name() == "button" {
            assert_eq!(elm.realized_size.x as i32, (width / 2 - (btn_width / 2 + spacing)) as i32 );
            assert_eq!(elm.realized_size.y as i32, spacing as i32);
        }
        found = true;
    }
    assert!(found);
}



#[test]
fn align_bottom_1_engine() {

    let width = 1000;
    let height = 600;

    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height).unwrap();

    let btn_height = 40;
    let spacing = 10;


    let col = Column::new()
        .width(Fill)
        .height(Fill)
        .padding(spacing as f32)
        .spacing(spacing as f32)
        .add(Row::new()
             .width(Fill)
             .height(FitContent)
             .align_bottom()
             .add_attribute(Attribute::Spacing(spacing as f32))
             .add(Button::new( "Right", Some(Message::Add))
                  .height(Px(btn_height as u32))
                  .align_left()
             )
        );

    let aligned_elements = engine::align_tree(col.into(), engine::Size { w: width as f32, h: height as f32 }, window.text_renderer());

    for elm in &aligned_elements {
        println!("{}.align = {:?}", elm.node.name(), elm.realized_size);
    }

    let mut found = false;
    for elm in &aligned_elements {
        if elm.node.name() == "button" {
            assert_eq!(elm.realized_size.x as i32, spacing as i32);
            assert_eq!(elm.realized_size.y as i32, (height - btn_height - spacing) as i32);
        }
        found = true;
    }

    assert!(found);
}



#[test]
fn align_bottom_2_engine() {

    let width = 1000;
    let height = 600;

    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height).unwrap();

    let btn_height = 40;
    let spacing = 10;

    let col = Row::new()
        .width(Fill)
        .height(Fill)
        .padding(spacing as f32)
        .spacing(spacing as f32)
        .add(Column::new()
             .width(Fill)
             .height(FitContent)
             .align_bottom()
             .add_attribute(Attribute::Spacing(spacing as f32))
             .add(Button::new("Right", Some(Message::Add))
                  .height(Px(btn_height as u32))
                  .width(Px(btn_height as u32))
                  .align_right()
             )
        );



    let aligned_elements = engine::align_tree(col.into(), engine::Size { w: width as f32, h: height as f32 }, window.text_renderer());

    for elm in &aligned_elements {
        println!("{}.align = {:?}", elm.node.name(), elm.realized_size);
    }

    let mut found = false;
    for elm in &aligned_elements {
        if elm.node.name() == "button" {
            assert_eq!(elm.realized_size.x as i32, (width - btn_height - spacing) as i32);
            assert_eq!(elm.realized_size.y as i32, (height - btn_height - spacing) as i32);
        }
        found = true;
    }

    assert!(found);
}
