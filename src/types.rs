#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}
