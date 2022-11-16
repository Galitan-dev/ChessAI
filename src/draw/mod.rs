use gfx_core::{command::Buffer, Factory, Resources};
use piston_window::{Context, Graphics, TextureContext};

mod board;
mod piece;

pub trait Drawable {
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
    ) -> ();
}
