use gfx_core::{command::Buffer, Factory, Resources};
use piston_window::{clear, rectangle, Context, Graphics, TextureContext, Transformed};

use super::Drawable;

use crate::game::{Board, BOARD_HEIGHT, BOARD_WIDTH};

impl Drawable for Board {
    fn draw<
        G: Graphics<Texture = piston_window::Texture<R>>,
        F: Factory<R>,
        R: Resources,
        C: Buffer<R>,
    >(
        &self,
        c: Context,
        g: &mut G,
        size: [f64; 2],
        tc: &mut TextureContext<F, R, C>,
    ) {
        clear([240. / 255., 218. / 255., 181. / 255., 1.], g);

        let w = size[0] / BOARD_WIDTH as f64;
        let h = size[1] / BOARD_HEIGHT as f64;
        let rect = rectangle::rectangle_by_corners(0.0, 0.0, w, h);

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let transform = c.transform.trans(x as f64 * w, y as f64 * h);

                if x % 2 != y % 2 {
                    rectangle(
                        [165. / 255., 123. / 255., 90. / 255., 1.],
                        rect,
                        transform,
                        g,
                    );
                }

                if self.selection() == Some([x, y]) {
                    rectangle(
                        [86. / 255., 130. / 255., 89. / 255., 0.7],
                        rect,
                        transform,
                        g,
                    );
                }
            }
        }

        for (x, y, piece) in self.pieces() {
            (x, y, piece, self.orientation()).draw(c, g, size, tc);
        }
    }
}
