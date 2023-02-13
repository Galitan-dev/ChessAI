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
    selected: Option<usize>,
    dragging: bool,
}

impl Board {
    pub fn get_piece(&self, x: usize, y: usize) -> Piece {
        self.pieces[y * 8 + x]
    }

    pub fn is_selected(&self, x: usize, y: usize) -> bool {
        self.selected == Some(y * 8 + x)
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging
    }

    pub fn get_selected(&self) -> Piece {
        self.selected.map(|i| self.pieces[i]).unwrap_or(Piece::None)
    }

    pub fn mouse_press(&mut self, mouse_x: f64, mouse_y: f64) {
        let x = (mouse_x * 8.).floor() as usize;
        let y = (mouse_y * 8.).floor() as usize;
        let square_index = y * 8 + x;

        if let Some(selected) = self.selected {
            self.selected = None;
            if selected != square_index {
                self.move_piece(selected, square_index);
            }
        } else {
            if self.pieces[square_index] == Piece::None {
                return;
            }

            self.selected = Some(square_index);
            self.dragging = true;
        }
    }

    pub fn mouse_relase(&mut self, mouse_x: f64, mouse_y: f64) {
        let x = (mouse_x * 8.).floor() as usize;
        let y = (mouse_y * 8.).floor() as usize;
        let square_index = y * 8 + x;

        self.dragging = false;
        if let Some(selected) = self.selected {
            if selected != square_index {
                self.move_piece(selected, square_index);
                self.selected = None;
            }
        }
    }

    pub fn move_piece(&mut self, from: usize, to: usize) {
        self.pieces.swap(from, to);
        self.pieces[from] = Piece::None;
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

        Self {
            pieces,
            selected: None,
            dragging: false,
        }
    }
}
