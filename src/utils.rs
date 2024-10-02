use crate::board::{Board, BOARD_SIZE};

impl Board {
    pub fn index_to_chess_notation(row: usize, col: usize) -> String {
        let file = (b'a' + col as u8) as char;
        let rank = (BOARD_SIZE - row).to_string();
        format!("{}{}", file, rank)
    }

    pub fn chess_notation_to_index(notation: &str) -> Option<(usize, usize)> {
        if notation.len() == 2 {
            let chars: Vec<char> = notation.chars().collect();
            let file = chars[0];
            let rank = chars[1];

            if ('a'..='h').contains(&file) && ('1'..='8').contains(&rank) {
                let col = (file as u8 - b'a') as usize;
                let row = BOARD_SIZE - (rank as usize - '0' as usize);
                return Some((row, col));
            }
        }
        None
    }
}
