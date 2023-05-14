use crate::na::{self, Translation3};
use crate::widget_gui::*;
use crate::text_rendering::{text_renderer::TextRenderer};
use crate::{gl::{self, viewport}, ScreenBox, ScreenCoords};
use crate::shader::{ TransformationShader, rounded_rect_shader::{self as rrs, RoundedRectShader}, circle_shader::{self as cs, CircleShader}};
use crate::objects::square;
use crate::color::Color;


pub struct RenderContext<'a> {
    pub gl: &'a gl::Gl,
    pub tr: &'a mut TextRenderer,
    pub viewport: &'a viewport::Viewport,
    pub render_square: &'a square::Square,
    pub rounded_rect_shader: &'a RoundedRectShader,
    pub circle_shader: &'a CircleShader,
}

pub fn render_ui(state: &UiState, ctx: &mut RenderContext) {

    // render as a list, not as a tree, might have to change that so parent are rendered before children
    // also in future layers with popup on top of other widgets
    // for now just assume all is in 1 layer and we can render in any order
    for id in 0..state.widgets.len() {
        state.widgets[id].render(&state.geom[id], ctx);
    }
}


pub fn transform_to_screen_space(geom: &Geometry, viewport: &viewport::Viewport) -> ScreenBox {
    ScreenBox::new(geom.pos.x as f32,
                   geom.pos.y as f32,
                   geom.size.pixel_w as f32,
                   geom.size.pixel_h as f32,
                   viewport.w as f32,
                   viewport.h as f32)
}



pub fn render_text(text: &str, scale: f32, geom: &Geometry, ctx: &mut RenderContext) {

    let sb = transform_to_screen_space(geom, &ctx.viewport);

    //ctx.tr.setup_blend(ctx.gl);
    //println!("{:?}", sb);
    //println!("{:?}", geom);

    ctx.tr.render_text(ctx.gl, &text, Default::default(), sb, 32);
}


pub fn render_circle(geom: &Geometry, radius: Pixel, ctx: &mut RenderContext) {

    ctx.circle_shader.shader.set_used();

    let transform = unit_square_transform_matrix(geom, ctx);

    ctx.circle_shader.set_transform(transform);

    let color = Color::Rgb(150, 150, 200);
    ctx.circle_shader.set_uniforms(cs::Uniforms { color,
                                              pixel_height: geom.size.pixel_h as f32,
                                              pixel_width: geom.size.pixel_w as f32,
                                              radius: radius as f32
    });


    ctx.render_square.render(ctx.gl);


}


pub fn render_round_rect(geom: &Geometry, ctx: &mut RenderContext) {
    ctx.rounded_rect_shader.shader.set_used();

    let transform = unit_square_transform_matrix(geom, ctx);

    ctx.rounded_rect_shader.set_transform(transform);

    let color = Color::Rgb(255, 200, 200);
    ctx.rounded_rect_shader.set_uniforms(rrs::Uniforms { color,
                                                         pixel_height: geom.size.pixel_h as f32,
                                                         pixel_width: geom.size.pixel_w as f32,
                                                         radius: 20.0
    });


    ctx.render_square.render(ctx.gl);
}



pub fn render_rect(geom: &Geometry, ctx: &mut RenderContext) {
    ctx.rounded_rect_shader.shader.set_used();

    let transform = unit_square_transform_matrix(geom, ctx);

    ctx.rounded_rect_shader.set_transform(transform);

    let color = Color::Rgb(255, 200, 200);
    ctx.rounded_rect_shader.set_uniforms(rrs::Uniforms { color,
                                                         pixel_height: geom.size.pixel_h as f32,
                                                         pixel_width: geom.size.pixel_w as f32,
                                                         radius: 0.0
    });


    ctx.render_square.render(ctx.gl);
}



pub fn unit_square_transform_matrix(geom: &Geometry, ctx: &RenderContext) -> na::Matrix4::<f32> {

    let sc_top_left = window_to_screen_coords(geom.pos.x as f32, geom.pos.y as f32, ctx.viewport.w as f32, ctx.viewport.h as f32);

    let x_scale = geom.size.pixel_w as f32 / ctx.viewport.w as f32 * 2.0;
    let y_scale = geom.size.pixel_h as f32  / ctx.viewport.h as f32 * 2.0;

    let mut model = na::Matrix4::<f32>::identity();

    // Scale to size
    model[0] = x_scale;
    model[5] = y_scale;

    // move to position

    let x_move = sc_top_left.x + x_scale * 0.5;
    let y_move = sc_top_left.y - y_scale * 0.5;

    let trans = Translation3::new(x_move, y_move, 0.0);

    model = trans.to_homogeneous() * model;

    model
}



pub fn window_to_screen_coords(x: f32, y: f32, w: f32, h: f32) -> ScreenCoords {
    ScreenCoords {x : x *2.0/ w  - 1.0, y: -y *2.0 / h + 1.0 }
}
