use find_folder::Search;
use gfx_core::{command::Buffer, Factory, Resources};
use piston_window::{image, Context, Flip, Graphics, Texture, TextureContext, TextureSettings};

use crate::game::Piece;

use super::Drawable;

impl Drawable for Piece {
    fn draw<
        G: Graphics<Texture = piston_window::Texture<R>>,
        F: Factory<R>,
        R: Resources,
        C: Buffer<R>,
    >(
        &self,
        c: Context,
        g: &mut G,
        _size: [f64; 2],
        tc: &mut TextureContext<F, R, C>,
    ) {
        let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let piece = assets
            .join("pieces")
            .join(self.color().id())
            .join(self.kind().id().to_owned() + ".png");

        let piece: Texture<R> =
            Texture::from_path(tc, piece, Flip::None, &TextureSettings::new()).unwrap();

        image(&piece, c.transform, g);
    }
}
