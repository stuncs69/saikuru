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
                            PieceType::Knight => self.generate_knight_moves(row, col, &mut moves),
                            PieceType::Bishop => self.generate_bishop_moves(row, col, &mut moves),
                            PieceType::Queen => self.generate_queen_moves(row, col, &mut moves),
                            PieceType::King => self.generate_king_moves(row, col, &mut moves),
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

    fn generate_knight_moves(&self, row: usize, col: usize, moves: &mut Vec<(usize, usize, usize, usize)>) {
        let knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2),
        ];

        for (dx, dy) in knight_moves.iter() {
            let new_row = row as isize + *dx;
            let new_col = col as isize + *dy;
            if new_row >= 0 && new_row < BOARD_SIZE as isize && new_col >= 0 && new_col < BOARD_SIZE as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                if self.grid[new_row][new_col].map_or(true, |p| p.color != self.grid[row][col].unwrap().color) {
                    moves.push((row, col, new_row, new_col));
                }
            }
        }
    }

    fn generate_bishop_moves(&self, row: usize, col: usize, moves: &mut Vec<(usize, usize, usize, usize)>) {
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        for (dx, dy) in directions.iter() {
            let mut new_row = row as isize;
            let mut new_col = col as isize;

            while let Some(piece) = self.grid.get(new_row as usize).and_then(|r| r.get(new_col as usize)) {
                new_row += *dx;
                new_col += *dy;
                if new_row < BOARD_SIZE as isize && new_col < BOARD_SIZE as isize {
                    if piece.is_none() {
                        moves.push((row, col, new_row as usize, new_col as usize));
                    } else {
                        if piece.as_ref().unwrap().color != self.grid[row][col].unwrap().color {
                            moves.push((row, col, new_row as usize, new_col as usize));
                        }
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn generate_queen_moves(&self, row: usize, col: usize, moves: &mut Vec<(usize, usize, usize, usize)>) {
        self.generate_rook_moves(row, col, moves);
        self.generate_bishop_moves(row, col, moves);
    }

    fn generate_king_moves(&self, row: usize, col: usize, moves: &mut Vec<(usize, usize, usize, usize)>) {
        let king_moves = [
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];

        for (dx, dy) in king_moves.iter() {
            let new_row = row as isize + *dx;
            let new_col = col as isize + *dy;
            if new_row >= 0 && new_row < BOARD_SIZE as isize && new_col >= 0 && new_col < BOARD_SIZE as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                if self.grid[new_row][new_col].map_or(true, |p| p.color != self.grid[row][col].unwrap().color) {
                    moves.push((row, col, new_row, new_col));
                }
            }
        }
    }

    pub fn make_move(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        self.grid[to_row][to_col] = self.grid[from_row][from_col];
        self.grid[from_row][from_col] = None;
    }
}
