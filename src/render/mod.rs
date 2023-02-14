use std::collections::HashMap;

use piston_window::{types::Color, Context, G2d, G2dTexture, RenderArgs};

mod board;
pub mod piece;

pub trait Render {
    fn rgb<N: Into<f32>>(&self, r: N, g: N, b: N) -> Color {
        self.rgba(r, g, b, 1.0)
    }

    fn rgba<N: Into<f32>>(&self, r: N, g: N, b: N, a: f32) -> Color {
        [r.into() / 255.0, g.into() / 255.0, b.into() / 255.0, a]
    }

    fn render(
        &mut self,
        args: RenderArgs,
        c: Context,
        g: &mut G2d,
        texture_bank: &HashMap<u8, G2dTexture>,
        mouse_pos: [f64; 2],
    );
}
