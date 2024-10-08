use crate::board::{Board, BOARD_SIZE};
use crate::types::Color;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Deserialize)]
struct BoardRequest {
    board: [[Option<String>; BOARD_SIZE]; BOARD_SIZE],
    depth: usize,
    player: bool,
}

#[derive(Serialize)]
struct MoveResponse {
    best_move: Option<String>,
}

pub async fn start_server() {
    let board_state = Arc::new(Mutex::new(Board::new()));

    let board = warp::path("best_move")
        .and(warp::post())
        .and(warp::body::json())
        .map({
            let board_state = Arc::clone(&board_state);
            move |req: BoardRequest| {
            println!("started looking for best moves with depth {:?}", req.depth);

                let mut board = board_state.lock().unwrap();
                *board = Board::from_json(&req.board);

                let mut color = Color::White;
                if !req.player {
                    color = Color::Black;
                }

                let (_, best_move) = board.minimax(req.depth, color, true);
                let best_move_str = best_move.map(|(from_row, from_col, to_row, to_col)| {
                    format!(
                        "{} -> {}",
                        Board::index_to_chess_notation(from_row, from_col),
                        Board::index_to_chess_notation(to_row, to_col)
                    )
                });

                println!("Best move: {:?}", best_move_str);

                warp::reply::json(&MoveResponse { best_move: best_move_str })
            }
        });

    warp::serve(board).run(([127, 0, 0, 1], 3030)).await;
}
