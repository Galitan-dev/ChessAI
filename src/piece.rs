use std::ops::{BitAnd, BitOr, Sub};

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
    pub fn name(&self) -> &str {
        match self {
            Piece::Pawn => "pawn",
            Piece::LeftKnight => "left_knight",
            Piece::RightKnight => "right_knight",
            Piece::Bishop => "bishop",
            Piece::Rook => "rook",
            Piece::Queen => "queen",
            Piece::King => "king",
            _ => panic!("Unexpected piece: {:?}", self),
        }
    }
}
