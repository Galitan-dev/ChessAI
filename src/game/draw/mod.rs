use graphics::Context;
use opengl_graphics::GlGraphics;

mod board;

pub trait Drawable {
    fn draw(&self, c: Context, gl: &mut GlGraphics, size: [f64; 2]) -> ();
}
