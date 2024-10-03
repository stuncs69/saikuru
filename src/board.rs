use crate::types::{Color, Piece, PieceType};
use std::fmt;

pub const BOARD_SIZE: usize = 8;

#[derive(Clone)]
pub struct Board {
    pub grid: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        let mut board = [[None; BOARD_SIZE]; BOARD_SIZE];

        for i in 0..BOARD_SIZE {
            board[1][i] = Some(Piece { color: Color::White, kind: PieceType::Pawn });
            board[6][i] = Some(Piece { color: Color::Black, kind: PieceType::Pawn });
        }

        board[0][0] = Some(Piece { color: Color::White, kind: PieceType::Rook });
        board[0][7] = Some(Piece { color: Color::White, kind: PieceType::Rook });
        board[7][0] = Some(Piece { color: Color::Black, kind: PieceType::Rook });
        board[7][7] = Some(Piece { color: Color::Black, kind: PieceType::Rook });

        Board { grid: board }
    }

    pub fn from_json(board_json: &[[Option<String>; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        let mut board = [[None; BOARD_SIZE]; BOARD_SIZE];

        for (row_index, row) in board_json.iter().enumerate() {
            for (col_index, square) in row.iter().enumerate() {
                if let Some(piece_str) = square {
                    let color = if piece_str.starts_with('w') { Color::White } else { Color::Black };
                    let kind = match &piece_str[1..] {
                        "P" => PieceType::Pawn,
                        "R" => PieceType::Rook,
                        _ => continue,
                    };
                    board[row_index][col_index] = Some(Piece { color, kind });
                }
            }
        }

        Board { grid: board }
    }

    pub fn display(&self) {
        for row in self.grid.iter() {
            for square in row.iter() {
                match square {
                    Some(piece) => match piece.kind {
                        PieceType::Pawn => print!("P "),
                        PieceType::Rook => print!("R "),
                        PieceType::Knight => print!("N "),
                        PieceType::Bishop => print!("B "),
                        PieceType::Queen => print!("Q "),
                        PieceType::King => print!("K "),
                    },
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}
