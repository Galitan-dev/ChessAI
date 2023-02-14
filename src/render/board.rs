use std::collections::HashMap;

use piston_window::{
    clear, rectangle, rectangle::square, Context, G2d, G2dTexture, RenderArgs, Transformed,
};

use crate::{board::Board, piece::Piece};

use super::Render;

impl Render for Board {
    fn render(
        &mut self,
        args: RenderArgs,
        c: Context,
        g: &mut G2d,
        texture_bank: &HashMap<u8, G2dTexture>,
        mouse_pos: [f64; 2],
    ) {
        let square_side = args.window_size[0] / 8.;
        let is_dragging = self.is_dragging();
        let selected_piece_legal_moves = self.get_selected_piece_legal_moves();
        let flying_piece = self.flying_piece();

        clear(self.rgb::<u8>(0, 48, 73), g);

        for x in 0..8 {
            for y in 0..8 {
                let is_light_square = (x + y) % 2 == 0;
                let is_selected = self.is_selected(x, y);
                let is_legal_move = selected_piece_legal_moves.contains(&[x, y]);
                let is_in_last_move = self.is_in_last_move(x, y);
                let is_flying = flying_piece
                    .map(|(from, ..)| from == [x, y])
                    .unwrap_or(false);
                let is_in_promotion = self.is_in_promotion(x, y);

                let c = c
                    .clone()
                    .scale(square_side, square_side)
                    .trans(x as f64, y as f64);

                if is_light_square {
                    rectangle(
                        self.rgb::<u8>(234, 226, 183),
                        square(0.0, 0.0, 1.),
                        c.transform,
                        g,
                    );
                }

                if is_selected {
                    rectangle(
                        self.rgba::<u8>(247, 127, 0, 0.9),
                        square(0.0, 0.0, 1.),
                        c.transform,
                        g,
                    );
                } else if is_legal_move {
                    rectangle(
                        self.rgba::<u8>(214, 40, 40, 0.9),
                        square(0.0, 0.0, 1.),
                        c.transform,
                        g,
                    );
                } else if is_in_last_move {
                    rectangle(
                        self.rgba::<u8>(247, 127, 0, 0.7),
                        square(0.0, 0.0, 1.),
                        c.transform,
                        g,
                    );
                }

                if is_in_promotion {
                    let color = if y == 0 { Piece::White } else { Piece::Black };
                    let c = c.clone().scale(0.5, 0.5);

                    (Piece::Queen | color).render(args, c, g, texture_bank, mouse_pos);
                    #[rustfmt::skip] (Piece::Rook | color).render(args, c.clone().trans(1., 0.), g, texture_bank, mouse_pos);
                    #[rustfmt::skip] (Piece::Bishop | color).render(args, c.clone().trans(0., 1.), g, texture_bank, mouse_pos);
                    #[rustfmt::skip] (Piece::LeftKnight | color).render(args, c.clone().trans(1., 1.), g, texture_bank, mouse_pos);
                }

                if (!is_selected || !is_dragging) && !is_flying {
                    self.get_piece(x, y)
                        .render(args, c, g, texture_bank, mouse_pos);
                }
            }
        }

        if is_dragging {
            self.get_selected().render(
                args,
                c.clone()
                    .trans_pos(mouse_pos)
                    .scale(square_side, square_side)
                    .trans(-0.5, -0.5),
                g,
                texture_bank,
                mouse_pos,
            );
        }

        if let Some(([x, y], [current_y, current_x], _)) = flying_piece {
            self.get_piece(x, y).render(
                args,
                c.clone()
                    .scale(square_side, square_side)
                    .trans(current_x, current_y),
                g,
                texture_bank,
                mouse_pos,
            );
        }
    }
}
