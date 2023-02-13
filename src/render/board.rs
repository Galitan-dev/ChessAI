use piston_window::{
    clear, rectangle, rectangle::square, Context, G2d, G2dTextureContext, RenderArgs, Transformed,
};

use crate::board::Board;

use super::Render;

impl Render for Board {
    fn render(
        &self,
        args: RenderArgs,
        c: Context,
        g: &mut G2d,
        texture_context: &mut G2dTextureContext,
    ) {
        let square_side = args.draw_size[0] as f64 / 8.;

        clear([0.0; 4], g);

        for x in 0..8 {
            for y in 0..8 {
                let is_light_square = (x + y) % 2 == 0;
                let color = if is_light_square {
                    self.rgb::<u8>(234, 226, 183)
                } else {
                    self.rgb::<u8>(0, 48, 73)
                };

                let c = c
                    .clone()
                    .scale(square_side, square_side)
                    .trans(x as f64, y as f64);

                rectangle(color, square(0.0, 0.0, 1.), c.transform, g);

                self.get_piece(x, y).render(args, c, g, texture_context);
            }
        }
    }
}
