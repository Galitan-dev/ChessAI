use graphics::{
    clear,
    color::{BLACK, WHITE},
    rectangle, Transformed,
};

use crate::game::Board;

use super::Drawable;

impl Drawable for Board {
    fn draw(
        &self,
        c: graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        size: [f64; 2],
    ) -> () {
        clear(WHITE, gl);

        let cell_width = size[0] / self.width as f64;
        let cell_height = size[1] / self.height as f64;
        let cell = rectangle::rectangle_by_corners(0.0, 0.0, cell_width, cell_height);

        for x in 0..self.width {
            for y in 0..self.height {
                if x % 2 != y % 2 {
                    let transform = c
                        .transform
                        .trans(x as f64 * cell_width, y as f64 * cell_height);

                    rectangle(BLACK, cell, transform, gl);
                }
            }
        }
    }
}
