use std::collections::HashMap;

use piston_window::{
    clear, rectangle, rectangle::square, Context, G2d, G2dTexture, RenderArgs,
    Transformed,
};

use crate::board::Board;

use super::Render;

impl Render for Board {
    fn render(
        &self,
        args: RenderArgs,
        c: Context,
        g: &mut G2d,
        texture_bank: &HashMap<u8, G2dTexture>,
    ) {
        let square_side = args.window_size[0] / 8.;

        clear(self.rgb::<u8>(0, 48, 73), g);

        for x in 0..8 {
            for y in 0..8 {
                let is_light_square = (x + y) % 2 == 0;

                let c = c
                    .clone()
                    .scale(square_side, square_side)
                    .trans(x as f64, y as f64);

                if is_light_square {
                    rectangle(
                        self.rgb::<u8>(234, 226, 183),
                        square(0.0, 0.0, 1.),
                        c.transform,
                        g,
                    );
                }

                self.get_piece(x, y)
                    .render(args, c, g, texture_bank);
            }
        }
    }
}
