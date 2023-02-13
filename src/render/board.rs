use piston_window::{clear, rectangle, rectangle::square, types::Color, Transformed};

use crate::{board::Board, window};

use super::Render;

impl Render for Board {
    fn render(&self, c: piston_window::Context, g: &mut piston_window::G2d) {
        let square_side = window::SIDE / 8.;

        clear([0.0; 4], g);

        for y in 0..8 {
            for x in 0..8 {
                let y = y as f64;
                let x = x as f64;

                let is_light_square = (x + y) % 2. != 0.;
                let color = if is_light_square {
                    self.rgb::<i16>(234, 226, 183)
                } else {
                    self.rgb::<i16>(0, 48, 73)
                };

                rectangle(
                    color,
                    square(0.0, 0.0, square_side),
                    c.transform.trans(x * square_side, y * square_side),
                    g,
                );
            }
        }
    }
}
