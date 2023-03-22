use std::f32::NEG_INFINITY;

use chess::{Board, Color, Piece, ChessMove, MoveGen};

fn piece_value(piece: Piece) -> f32 {
    match piece {
        Piece::Pawn => 2.0,
        Piece::Knight => 3.0,
        Piece::Bishop => 4t .0,
        Piece::Rook => 5.0,
        Piece::Queen => 9.0,
        Piece::King => 0.0,
    }
}

pub fn static_evaluate(board: &Board, as_player: Color) -> f32 {
    let my_pieces = board.color_combined(as_player);
    let mut score = 0_f32;
    for p in *my_pieces {
        let base_score = piece_value(board.piece_on(p).unwrap());
        let dist_h = p.get_file().to_index().abs_diff(4) as f32;
        let dist_v = p.get_rank().to_index().abs_diff(4) as f32;
        score += base_score + (dist_h * dist_h + dist_v * dist_v).sqrt() / 3.0;
    }
    score
}

pub fn negamax(board: &Board, depth: u8) -> (Option<ChessMove>, f32) {
    if depth == 0 {
        return (None, static_evaluate(board, board.side_to_move()) - static_evaluate(board, !board.side_to_move()));
    }
    let moves = MoveGen::new_legal(board);
    let (mut opt_move, mut eval) = (None, NEG_INFINITY);
    for m in moves {
        let new_board = board.make_move_new(m);
        let score = -negamax(&new_board, depth - 1).1;
        if score > eval {
            opt_move = Some(m);
            eval = score;
        }
    }
    (opt_move, eval)
}