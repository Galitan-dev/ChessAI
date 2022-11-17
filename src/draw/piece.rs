use find_folder::Search;
use gfx_core::{command::Buffer, Factory, Resources};
use piston_window::{
    image, Context, Flip, Graphics, Texture, TextureContext, TextureSettings, Transformed,
};

use crate::game::{BoardOrientation, Piece, BOARD_HEIGHT, BOARD_WIDTH};

use super::Drawable;

impl Drawable for (usize, usize, Piece, BoardOrientation) {
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
        let &(x, y, piece, orientation) = self;

        let w = size[0] / BOARD_WIDTH as f64;
        let h = size[1] / BOARD_HEIGHT as f64;

        let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let piece = assets
            .join("pieces")
            .join(piece.color().id())
            .join(piece.kind().id().to_owned() + ".png");

        let transform = match orientation {
            BoardOrientation::White => c.transform.trans(x as f64 * w, y as f64 * h),
            BoardOrientation::Black => c.transform.trans(x as f64 * w, (7 - y) as f64 * h),
        }
        .scale(w / 60., h / 60.);

        let piece: Texture<R> =
            Texture::from_path(tc, piece, Flip::None, &TextureSettings::new()).unwrap();

        image(&piece, transform, g);
    }
}
