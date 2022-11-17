use super::BoardOrientation;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn id(&self) -> &str {
        match &self {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    color: PieceColor,
    kind: PieceKind,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self { kind, color }
    }

    pub fn color(&self) -> PieceColor {
        self.color
    }

    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    pub fn moves(&self, orientation: BoardOrientation, x: usize, y: usize) -> Vec<[isize; 2]> {
        let x = x as isize;
        let y = y as isize;

        self.kind()
            .moves()
            .into_iter()
            .map(|[dx, dy]| {
                let dx = dx as isize;
                let dy = dy as isize;

                if orientation == self.color() {
                    [x + dx, y - dy]
                } else {
                    [x + dx, y + dy]
                }
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Qween,
    King,
}

impl PieceKind {
    pub fn id(&self) -> &str {
        match self {
            Self::Pawn { .. } => "pawn",
            Self::Knight { .. } => "knight",
            Self::Bishop { .. } => "bishop",
            Self::Rook { .. } => "rook",
            Self::Qween { .. } => "qween",
            Self::King { .. } => "king",
        }
    }

    fn moves(&self) -> Vec<[isize; 2]> {
        match self {
            PieceKind::Pawn => vec![[0, 1], [0, 2]],
            PieceKind::Knight => vec![
                [1, 2],
                [-1, 2],
                [1, -2],
                [-1, -2],
                [2, 1],
                [-2, 1],
                [2, -1],
                [-2, -1],
            ],
            PieceKind::Bishop => {
                let mut moves = Vec::new();
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x != 0 && y != 0 {
                            for i in 1..=7 {
                                moves.push([i * x, i * y]);
                            }
                        }
                    }
                }
                moves
            }
            PieceKind::Rook => {
                let mut moves = Vec::new();
                for x in -1..=1 {
                    for y in -1..=1 {
                        if (x == 0) != (y == 0) {
                            for i in 1..=7 {
                                moves.push([i * x, i * y]);
                            }
                        }
                    }
                }
                moves
            }
            PieceKind::Qween => {
                let mut moves = Vec::new();
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x != 0 || y != 0 {
                            for i in 1..=7 {
                                moves.push([i * x, i * y]);
                            }
                        }
                    }
                }
                moves
            }
            PieceKind::King => {
                let mut moves = Vec::new();
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x != 0 || y != 0 {
                            moves.push([x, y]);
                        }
                    }
                }
                moves
            }
        }
    }
}
