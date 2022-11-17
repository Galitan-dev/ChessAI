use crate::utils::Array2D;

use super::piece::{Piece, PieceColor, PieceColor::*, PieceKind, PieceKind::*};

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_HEIGHT: usize = 8;

pub struct Board {
    pieces: Array2D<Piece, BOARD_WIDTH, BOARD_HEIGHT>,
    orientation: BoardOrientation,
    selection: Option<[usize; 2]>,
}

pub type BoardOrientation = PieceColor;

impl Board {
    pub fn new(orientation: BoardOrientation) -> Self {
        let mut board = Self {
            pieces: Array2D::new(),
            orientation,
            selection: None,
        };

        board.fill_pieces();

        board
    }

    pub fn selection(&self) -> Option<[usize; 2]> {
        self.selection
    }

    fn fill_pieces(&mut self) {
        for x in 0..BOARD_WIDTH {
            self.add(Pawn, [x, 6], White);
        }

        self.add(Bishop, [2, 7], White);
        self.add(Bishop, [5, 7], White);

        self.add(Rook, [0, 7], White);
        self.add(Rook, [7, 7], White);

        self.add(Knight, [1, 7], White);
        self.add(Knight, [6, 7], White);

        self.add(King, [4, 7], White);
        self.add(Qween, [3, 7], White);

        for x in 0..BOARD_WIDTH {
            self.add(Pawn, [x, 1], Black);
        }

        self.add(Bishop, [2, 0], Black);
        self.add(Bishop, [5, 0], Black);

        self.add(Rook, [0, 0], Black);
        self.add(Rook, [7, 0], Black);

        self.add(Knight, [1, 0], Black);
        self.add(Knight, [6, 0], Black);

        self.add(King, [4, 0], Black);
        self.add(Qween, [3, 0], Black);
    }

    pub fn add(&mut self, a: PieceKind, at: [usize; 2], with: PieceColor) {
        let [x, y] = at;
        self.pieces.set(x, y, Piece::new(a, with));
    }

    pub fn pieces(&self) -> Array2D<Piece, BOARD_WIDTH, BOARD_HEIGHT> {
        self.pieces
    }

    pub fn select(&mut self, x: usize, y: usize) {
        if self.pieces.has(x, y) {
            self.selection = Some([x, y]);
        }
    }

    pub fn orientation(&self) -> BoardOrientation {
        self.orientation
    }
}
