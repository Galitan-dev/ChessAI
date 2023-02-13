use crate::piece::Piece;

const FRONT_ROW: [Piece; 8] = [Piece::Pawn; 8];
const BACK_ROW: [Piece; 8] = [
    Piece::Rook,
    Piece::LeftKnight,
    Piece::Bishop,
    Piece::Queen,
    Piece::King,
    Piece::Bishop,
    Piece::RightKnight,
    Piece::Rook,
];

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pieces: [Piece; 64],
}

impl Board {
    pub fn get_piece(&self, x: usize, y: usize) -> Piece {
        self.pieces[y * 8 + x]
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut pieces = [Piece::None; 64];

        let black_pieces: [Piece; 16] =
            concat_arrays!(BACK_ROW, FRONT_ROW).map(|p| p | Piece::Black);
        let white_pieces: [Piece; 16] =
            concat_arrays!(FRONT_ROW, BACK_ROW).map(|p| p | Piece::White);

        black_pieces.clone().swap_with_slice(&mut pieces[..16]);
        white_pieces.clone().swap_with_slice(&mut pieces[48..]);

        Self { pieces }
    }
}
