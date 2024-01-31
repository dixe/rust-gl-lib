use super::*;

// grid of X * Y size
// can do a draw active, to make a shape.
// draw on active shape/layer

pub struct CellGrid<T> {
    data : Vec<GridCell<T>>,
    w: usize,
    h: usize,
    zoom: f32,
    cell_w: f32,
    cell_h: f32,
    hot_rect: Option<Rect>
}

impl<T> CellGrid<T> {

    pub fn new(w: usize, h: usize) -> Self {

        let mut data = vec![];
        for _ in 0..(w*h) {
            data.push(GridCell::Inactive);
        }

        Self {
            data,
            w,
            h,
            zoom: 1.0,
            cell_w: 20.0,
            cell_h: 20.0,
            hot_rect: None
        }
    }

}


impl Ui {

    pub fn cell_grid<T>(&mut self, rect: Rect, grid: &mut CellGrid<T>, paint: PaintType<T> ) {
        self.draw_grid(rect, grid, paint);
    }


    fn draw_grid<T>(&mut self, rect: Rect, grid: &CellGrid<T>, paint: PaintType<T>) {
        // start by drawing grid with top left in top left corner, stop when we reach rect

        // background color: button color, TODO: Have it on grid, or its own style at least
        self.drawer2D.rounded_rect_color(rect.x, rect.y , rect.w, rect.h, 0.0, self.style.button.color);

        let rect_w = rect.w as f32;
        let rect_h = rect.h as f32;

        let thick = 2.0;
        for r in 0..grid.h {
            // if we draw and it ends up outside of the rect, stop
            let y_start = r as f32 * grid.zoom * grid.cell_h;
            if y_start >= rect_h  {
                break;
            }
            for c in 0.. grid.w  {
                let x_start = c as f32 * grid.zoom * grid.cell_w;
                if x_start >= rect_w {
                    break;
                }

                // draw square for cell
                let cell = &grid.data[r * grid.w + c];

                let x = x_start + thick / 2.0 + rect.x as f32;
                let y = y_start + thick / 2.0 + rect.y as f32;
                let w = grid.cell_w - thick;
                let h = grid.cell_h - thick;


                let click_rect = Rect {
                    x : x as i32,
                    y : y as i32,
                    w : w as i32,
                    h : h as i32
                };


                // TODO: Also do the hot and active states like button
                if self.mouse_in_rect(&click_rect) {
                    if self.mouse_up {
                        self.drawer2D.rounded_rect_color(x, y, w, h, 0.0, Color::white());
                    }
                }
                else {
                    self.drawer2D.rounded_rect_color(x, y, w, h, 0.0, Color::black());
                }
            }
        }

    }
}


pub enum GridCell<T> {
    Inactive,
    Active(T)
}


pub enum PaintType<T> {
    Activate,
    Deactivate,
    Data(T)
}
