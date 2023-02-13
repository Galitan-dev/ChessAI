use piston_window::{Flip, G2d, G2dTextureContext, RenderArgs, Texture, TextureSettings, Image, rectangle::square};

use crate::piece::Piece;

use super::Render;

impl Render for Piece {
    fn render(
        &self,
        _: RenderArgs,
        c: piston_window::Context,
        g: &mut G2d,
        texture_context: &mut G2dTextureContext,
    ) {
        if *self == Piece::None {
            return;
        }

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let pieces = assets.join("pieces");
        let color_pieces = pieces.join(if self.is_white() { "white" } else { "black" });
        let piece = color_pieces.join(format!("{}.png", self.name()));
        
        let image = Image::new().rect(square(0.0, 0.0, 1.0));
        let texture =
            Texture::from_path(texture_context, piece, Flip::None, &TextureSettings::new())
                .unwrap();

        image.draw(&texture, &c.draw_state, c.transform, g);
    }
}
