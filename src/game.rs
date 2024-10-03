use crate::board::{Board, BOARD_SIZE};
use crate::types::{Color, PieceType};

impl Board {
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.grid[row][col] {
                    let piece_value = match piece.kind {
                        PieceType::Pawn => 1,
                        PieceType::Rook => 5,
                        PieceType::Knight => 3,
                        PieceType::Bishop => 3,
                        PieceType::Queen => 9,
                        PieceType::King => 1000,
                    };
                    score += match piece.color {
                        Color::White => piece_value,
                        Color::Black => -piece_value,
                    };
                }
            }
        }
        score
    }

    pub fn minimax(&self, depth: usize, color: Color, maximizing_player: bool) -> (i32, Option<(usize, usize, usize, usize)>) {
        if depth == 0 {
            return (self.evaluate(), None);
        }

        let moves = self.generate_moves(color);

        if maximizing_player {
            let mut max_eval = i32::MIN;
            let mut best_move = None;

            for &(from_row, from_col, to_row, to_col) in &moves {
                let mut board_copy = self.clone();
                board_copy.make_move(from_row, from_col, to_row, to_col);
                let (eval, _) = board_copy.minimax(depth - 1, opposite_color(color), false);
                if eval > max_eval {
                    max_eval = eval;
                    best_move = Some((from_row, from_col, to_row, to_col));
                }
            }

            (max_eval, best_move)
        } else {
            let mut min_eval = i32::MAX;
            let mut best_move = None;

            for &(from_row, from_col, to_row, to_col) in &moves {
                let mut board_copy = self.clone();
                board_copy.make_move(from_row, from_col, to_row, to_col);
                let (eval, _) = board_copy.minimax(depth - 1, opposite_color(color), true);
                if eval < min_eval {
                    min_eval = eval;
                    best_move = Some((from_row, from_col, to_row, to_col));
                }
            }

            (min_eval, best_move)
        }
    }
}

fn opposite_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}
