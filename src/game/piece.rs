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
