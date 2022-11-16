use gfx_core::{command::Buffer, Factory, Resources};
use piston_window::{clear, rectangle, Context, Graphics, TextureContext, Transformed};

use super::Drawable;

use crate::game::{Board, BoardOrientation, BOARD_HEIGHT, BOARD_WIDTH};

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
                if x % 2 != y % 2 {
                    let transform = c.transform.trans(x as f64 * w, y as f64 * h);

                    rectangle(
                        [165. / 255., 123. / 255., 90. / 255., 1.],
                        rect,
                        transform,
                        g,
                    );
                }
            }
        }

        for piece in self.pieces() {
            let c = match self.orientation() {
                BoardOrientation::White => {
                    c.trans(piece.positionf64()[0] * w, piece.positionf64()[1] * h)
                }
                BoardOrientation::Black => c.trans(
                    piece.positionf64()[0] * w,
                    (7. - piece.positionf64()[1]) * h,
                ),
            }
            .scale(w / 60., h / 60.);

            piece.draw(c, g, size, tc);
        }
    }
}
