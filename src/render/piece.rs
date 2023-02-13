use std::collections::HashMap;

use piston_window::{
    rectangle::square, Flip, G2d, G2dTexture, G2dTextureContext, Image, RenderArgs, TextureSettings,
};

use crate::piece::Piece;

use super::Render;

impl Render for Piece {
    fn render(
        &self,
        _args: RenderArgs,
        c: piston_window::Context,
        g: &mut G2d,
        texture_bank: &HashMap<u8, G2dTexture>,
        _mouse_pos: [f64; 2],
    ) {
        if *self == Piece::None {
            return;
        }

        let image = Image::new().rect(square(0.0, 0.0, 1.0));
        let texture = texture_bank.get(&(*self as u8)).unwrap();

        image.draw(texture, &c.draw_state, c.transform, g);
    }
}

pub fn texture_bank(texture_context: &mut G2dTextureContext) -> HashMap<u8, G2dTexture> {
    let mut bank = HashMap::new();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let pieces = assets.join("pieces");
    let white_pieces = pieces.join("white");
    let black_pieces = pieces.join("black");

    for i in 1..8 {
        let piece: Piece = num::FromPrimitive::from_u8(i).unwrap();
        let file = format!("{}.png", piece.name());

        bank.insert(
            i | Piece::White as u8,
            G2dTexture::from_path(
                texture_context,
                white_pieces.join(file.clone()),
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        );
        bank.insert(
            i | Piece::Black as u8,
            G2dTexture::from_path(
                texture_context,
                black_pieces.join(file.clone()),
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        );
    }

    bank
}
