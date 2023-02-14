use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opponent {
    Player,
    Computer
}

#[derive(Debug, Clone)]
pub struct Board {
    pieces: [Piece; 64],
    selected: Option<usize>,
    dragging: bool,
    last_move: [usize; 2],
    moved_pieces: HashSet<usize>,
    current_turn: Piece,
    white_opponent: Opponent,
    black_opponent: Opponent,
}

impl Board {
    pub fn get_piece_maybe(&self, x: isize, y: isize) -> Option<Piece> {
        if x < 0 || y < 0 {
            return None;
        }

        self.pieces.get((y * 8 + x) as usize).copied()
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Piece {
        self.pieces[y * 8 + x]
    }

    pub fn is_selected(&self, x: usize, y: usize) -> bool {
        self.selected == Some(y * 8 + x)
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging
    }

    pub fn get_last_move(&self) -> [usize; 2] {
        self.last_move
    }

    pub fn get_selected(&self) -> Piece {
        self.selected.map(|i| self.pieces[i]).unwrap_or(Piece::None)
    }

    pub fn get_selected_piece_legal_moves(&self) -> Vec<[usize; 2]> {
        self.selected
            .map(|i| self.get_legal_moves(i))
            .unwrap_or_default()
    }

    fn get_legal_moves(&self, square_index: usize) -> Vec<[usize; 2]> {
        self.pieces[square_index].legal_moves(
            [
                square_index % 8,
                (square_index as f64 / 8.).floor() as usize,
            ],
            self,
        )
    }

    fn get_legal_moves_square_indices(&self, square_index: usize) -> Vec<usize> {
        self.get_legal_moves(square_index)
            .iter()
            .map(|[x, y]| y * 8 + x)
            .collect()
    }

    pub fn is_in_last_move(&self, x: usize, y: usize) -> bool {
        if self.last_move[0] == self.last_move[1] {
            return false;
        }

        let square_index = y * 8 + x;
        self.last_move[0] == square_index || self.last_move[1] == square_index
    }

    pub fn piece_has_moved(&self, x: usize, y: usize) -> bool {
        self.moved_pieces.contains(&(y * 8 + x))
    }

    pub fn current_opponent(&self) -> Opponent {
        if self.current_turn.is_white() {
            self.white_opponent
        } else {
            self.black_opponent
        }
    }

    pub fn mouse_press(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.current_opponent() != Opponent::Player {
            return;
        }

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
        if self.current_opponent() != Opponent::Player {
            return;
        }

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
        if self.get_legal_moves_square_indices(from).contains(&to) {
            let (piece, _) = self.pieces[from].split();

            let is_little_castle = piece == Piece::King && to == from + 2;
            let is_big_castle = piece == Piece::King && to == from - 2;
            let is_en_passant =
                piece == Piece::Pawn && to % 8 != from % 8 && self.pieces[to].is_none();

            self.pieces.swap(from, to);
            self.pieces[from] = Piece::None;
            self.last_move = [from, to];
            self.moved_pieces.insert(from);
            self.moved_pieces.insert(to);

            if is_little_castle {
                self.pieces.swap(from + 3, from + 1);
                self.moved_pieces.insert(from + 3);
            }
            if is_big_castle {
                self.pieces.swap(from - 4, from - 1);
                self.moved_pieces.insert(from - 4);
            }
            if is_en_passant {
                self.pieces[from + to % 8 - from % 8] = Piece::None;
                self.moved_pieces.insert(from + to % 8 - from % 8);
            }

            self.current_turn = self.current_turn.ennemy()
        }
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
            last_move: [0; 2],
            moved_pieces: HashSet::new(),
            current_turn: Piece::White,
            white_opponent: Opponent::Player,
            black_opponent: Opponent::Player
        }
    }
}
