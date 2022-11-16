#[derive(Clone, Copy)]
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
    position: PiecePosition,
    kind: PieceKind,
}

impl Piece {
    pub fn new(kind: PieceKind, position: PiecePosition, color: PieceColor) -> Self {
        Self {
            kind,
            position,
            color,
        }
    }

    pub fn position(&self) -> PiecePosition {
        self.position
    }

    pub fn positionf64(&self) -> [f64; 2] {
        [self.position()[0] as f64, self.position()[1] as f64]
    }

    pub fn color(&self) -> PieceColor {
        self.color
    }

    pub fn kind(&self) -> PieceKind {
        self.kind
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
}

pub type PiecePosition = [usize; 2];
