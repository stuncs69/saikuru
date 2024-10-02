use crate::board::{Board, BOARD_SIZE};
use crate::types::{Color, PieceType};

impl Board {
    pub fn generate_moves(&self, color: Color) -> Vec<(usize, usize, usize, usize)> {
        let mut moves = Vec::new();

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.grid[row][col] {
                    if piece.color == color {
                        match piece.kind {
                            PieceType::Pawn => self.generate_pawn_moves(row, col, &mut moves),
                            PieceType::Rook => self.generate_rook_moves(row, col, &mut moves),
                        }
                    }
                }
            }
        }

        moves
    }

    fn generate_pawn_moves(&self, row: usize, col: usize, moves: &mut Vec<(usize, usize, usize, usize)>) {
        if let Some(piece) = self.grid[row][col] {
            if piece.color == Color::White {
                if row < 7 && self.grid[row + 1][col].is_none() {
                    moves.push((row, col, row + 1, col));
                }
            } else {
                if row > 0 && self.grid[row - 1][col].is_none() {
                    moves.push((row, col, row - 1, col));
                }
            }
        }
    }

    fn generate_rook_moves(&self, row: usize, col: usize, moves: &mut Vec<(usize, usize, usize, usize)>) {
        for i in (0..row).rev() {
            if self.grid[i][col].is_none() {
                moves.push((row, col, i, col));
            } else {
                break;
            }
        }
        for i in row + 1..BOARD_SIZE {
            if self.grid[i][col].is_none() {
                moves.push((row, col, i, col));
            } else {
                break;
            }
        }
        for i in (0..col).rev() {
            if self.grid[row][i].is_none() {
                moves.push((row, col, row, i));
            } else {
                break;
            }
        }
        for i in col + 1..BOARD_SIZE {
            if self.grid[row][i].is_none() {
                moves.push((row, col, row, i));
            } else {
                break;
            }
        }
    }

    pub fn make_move(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        self.grid[to_row][to_col] = self.grid[from_row][from_col];
        self.grid[from_row][from_col] = None;
    }
}
