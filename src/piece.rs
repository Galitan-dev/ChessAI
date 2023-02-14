use std::ops::{BitAnd, BitOr, Range, Sub};

use crate::board::Board;

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Piece {
    None = 0,

    Pawn = 1,
    LeftKnight = 2,
    RightKnight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
    King = 7,

    White = 8,
    Black = 16,

    WhitePawn = 1 | 8,
    WhiteLeftKnight = 2 | 8,
    WhiteRightKnight = 3 | 8,
    WhiteBishop = 4 | 8,
    WhiteRook = 5 | 8,
    WhiteQueen = 6 | 8,
    WhiteKing = 7 | 8,

    BlackPawn = 1 | 16,
    BlackLeftKnight = 2 | 16,
    BlackRightKnight = 3 | 16,
    BlackBishop = 4 | 16,
    BlackRook = 5 | 16,
    BlackQueen = 6 | 16,
    BlackKing = 7 | 16,
}

impl BitOr for Piece {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        num::FromPrimitive::from_u8(self as u8 | rhs as u8).unwrap_or(Piece::None)
    }
}

impl BitAnd for Piece {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        num::FromPrimitive::from_u8(self as u8 & rhs as u8).unwrap_or(Piece::None)
    }
}

impl Sub for Piece {
    type Output = Piece;

    fn sub(self, rhs: Self) -> Self::Output {
        num::FromPrimitive::from_u8(self as u8 - rhs as u8).unwrap_or(Piece::None)
    }
}

impl Piece {
    pub fn is_none(&self) -> bool {
        *self == Self::None
    }

    pub fn is_white(&self) -> bool {
        *self & Self::White == Self::White
    }

    pub fn is_black(&self) -> bool {
        *self & Self::Black == Self::Black
    }

    pub fn ennemy(&self) -> Self {
        if self.is_white() {
            Self::Black
        } else if self.is_black() {
            Self::White
        } else {
            Self::None
        }
    }

    pub fn split(&self) -> (Self, Self) {
        let color = self.color();
        let piece = *self - color;

        (piece, color)
    }

    pub fn color(&self) -> Self {
        if self.is_white() {
            Self::White
        } else if self.is_black() {
            Self::Black
        } else {
            Self::None
        }
    }

    pub fn legal_moves(&self, [ux, uy]: [usize; 2], board: &Board) -> Vec<[usize; 2]> {
        let x = ux as isize;
        let y = uy as isize;

        let mut moves = Vec::new();
        let (piece, _) = self.split();

        match piece {
            Self::Rook => {
                self.slide_to_wall(x..-1, y..0, board, &mut moves);
                self.slide_to_wall(x..1, y..0, board, &mut moves);
                self.slide_to_wall(x..0, y..-1, board, &mut moves);
                self.slide_to_wall(x..0, y..1, board, &mut moves);
            }
            Self::Bishop => {
                self.slide_to_wall(x..-1, y..-1, board, &mut moves);
                self.slide_to_wall(x..1, y..1, board, &mut moves);
                self.slide_to_wall(x..1, y..-1, board, &mut moves);
                self.slide_to_wall(x..-1, y..1, board, &mut moves);
            }
            Self::Queen => {
                self.slide_to_wall(x..-1, y..0, board, &mut moves);
                self.slide_to_wall(x..1, y..0, board, &mut moves);
                self.slide_to_wall(x..0, y..-1, board, &mut moves);
                self.slide_to_wall(x..0, y..1, board, &mut moves);
                self.slide_to_wall(x..-1, y..-1, board, &mut moves);
                self.slide_to_wall(x..1, y..1, board, &mut moves);
                self.slide_to_wall(x..1, y..-1, board, &mut moves);
                self.slide_to_wall(x..-1, y..1, board, &mut moves);
            }
            Self::King => {
                self.slide([x + 1, y + 1], board, &mut moves);
                let gone_right = self.slide([x + 1, y], board, &mut moves);
                self.slide([x + 1, y - 1], board, &mut moves);
                self.slide([x, y + 1], board, &mut moves);
                self.slide([x, y - 1], board, &mut moves);
                self.slide([x - 1, y + 1], board, &mut moves);
                let gone_left = self.slide([x - 1, y], board, &mut moves);
                self.slide([x - 1, y - 1], board, &mut moves);

                if !board.piece_has_moved(ux, uy) {
                    if gone_right && !board.piece_has_moved(7, uy) {
                        self.slide([x + 2, y], board, &mut moves);
                    }
                    if gone_left
                        && !board.piece_has_moved(0, uy)
                        && board.get_piece(1, uy).is_none()
                    {
                        self.slide([x - 2, y], board, &mut moves);
                    }
                }
            }
            Self::Pawn => {
                let dy = if self.is_white() { -1 } else { 1 };
                if board.get_piece(ux, (y + dy) as usize).is_none() {
                    moves.push([ux, (y + dy) as usize]);

                    if y == if self.is_white() { 6 } else { 1 }
                        && board.get_piece(ux, (y + 2 * dy) as usize).is_none()
                    {
                        moves.push([ux, (y + 2 * dy) as usize]);
                    }
                }

                if x > 0
                    && (board.get_piece(ux - 1, (y + dy) as usize).color() == self.ennemy()
                        || (board.get_piece(ux - 1, y as usize).split()
                            == (Self::Pawn, self.ennemy())
                            && board.get_last_move()
                                == [
                                    ((y + 2 * dy) * 8 + x - 1) as usize,
                                    (y * 8 + x - 1) as usize,
                                ]))
                {
                    moves.push([ux - 1, (y + dy) as usize]);
                }

                if x < 7
                    && (board.get_piece(ux + 1, (y + dy) as usize).color() == self.ennemy()
                        || (board.get_piece(ux + 1, y as usize).split()
                            == (Self::Pawn, self.ennemy())
                            && board.get_last_move()
                                == [
                                    ((y + 2 * dy) * 8 + x + 1) as usize,
                                    (y * 8 + x + 1) as usize,
                                ]))
                {
                    moves.push([ux + 1, (y + dy) as usize]);
                }
            }
            Self::LeftKnight | Self::RightKnight => {
                self.slide([x + 2, y + 1], board, &mut moves);
                self.slide([x + 2, y - 1], board, &mut moves);
                self.slide([x - 2, y + 1], board, &mut moves);
                self.slide([x - 2, y - 1], board, &mut moves);
                self.slide([x + 1, y + 2], board, &mut moves);
                self.slide([x - 1, y + 2], board, &mut moves);
                self.slide([x + 1, y - 2], board, &mut moves);
                self.slide([x - 1, y - 2], board, &mut moves);
            }
            _ => (),
        }

        moves
    }

    pub fn slide_to_wall(
        &self,
        x_way: Range<isize>,
        y_way: Range<isize>,
        board: &Board,
        moves: &mut Vec<[usize; 2]>,
    ) {
        let (mut x, dx) = (x_way.start, x_way.end);
        let (mut y, dy) = (y_way.start, y_way.end);

        x += dx;
        y += dy;

        while (0..8).contains(&x) && (0..8).contains(&y) {
            if !self.slide([x, y], board, moves) {
                break;
            }

            x += dx;
            y += dy;
        }
    }

    pub fn slide(&self, [x, y]: [isize; 2], board: &Board, moves: &mut Vec<[usize; 2]>) -> bool {
        if let Some(color) = board.get_piece_maybe(x, y).map(|p| p.color()) {
            if color != self.color() {
                moves.push([x as usize, y as usize]);
            }
            color.is_none()
        } else {
            false
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Pawn => "pawn",
            Self::LeftKnight => "left_knight",
            Self::RightKnight => "right_knight",
            Self::Bishop => "bishop",
            Self::Rook => "rook",
            Self::Queen => "queen",
            Self::King => "king",
            _ => panic!("Unexpected piece: {:?}", self),
        }
    }
}
