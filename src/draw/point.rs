use piston_window::{ellipse, Transformed};

use crate::game::{BOARD_HEIGHT, BOARD_WIDTH};

use super::Drawable;

impl Drawable for [usize; 2] {
    fn draw<
        G: piston_window::Graphics<Texture = piston_window::Texture<R>>,
        F: gfx_core::Factory<R>,
        R: gfx_core::Resources,
        C: gfx_core::command::Buffer<R>,
    >(
        &self,
        c: piston_window::Context,
        g: &mut G,
        size: [f64; 2],
        _tc: &mut piston_window::TextureContext<F, R, C>,
    ) -> () {
        let &[x, y] = self;

        let w = size[0] / BOARD_WIDTH as f64;
        let h = size[1] / BOARD_HEIGHT as f64;

        let transform = c
            .transform
            .trans(x as f64 * w + w / 4., y as f64 * h + h / 4.);

        let rect = [0., 0., w * 0.5, h * 0.5];

        ellipse([0.1, 0.1, 0.1, 0.9], rect, transform, g)
    }
}
