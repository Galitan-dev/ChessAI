use piston_window::{types::Color, Context, G2d, G2dTextureContext, RenderArgs};

mod board;
mod piece;

pub trait Render {
    fn rgb<N: Into<f32>>(&self, r: N, g: N, b: N) -> Color {
        [r.into() / 255.0, g.into() / 255.0, b.into() / 255.0, 1.0]
    }

    fn render(
        &self,
        args: RenderArgs,
        c: Context,
        g: &mut G2d,
        texture_context: &mut G2dTextureContext,
    );
}
