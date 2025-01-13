use crate::game::{Colour, Game, PieceType};
use crate::moves::{generate_moves, test_move, Move};
use crate::utils::bit_to_onebit_index;

static PAWN_PST: [i32; 64] =
    [0,  0,  0,  0,  0,  0,  0,  0,
        50, 50, 50, 50, 50, 50, 50, 50,
        10, 10, 20, 30, 30, 20, 10, 10,
        5,  5, 10, 25, 25, 10,  5,  5,
        0,  0,  0, 20, 20,  0,  0,  0,
        5, -5,-10,  0,  0,-10, -5,  5,
        5, 10, 10,-20,-20, 10, 10,  5,
        0,  0,  0,  0,  0,  0,  0,  0];

static KNIGHT_PST: [i32; 64] =
    [-50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50];

static BISHOP_PST: [i32; 64] =
    [-20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20];

static ROOK_PST: [i32; 64] =
    [0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  5,  5,  0,  0,  0];

static QUEEN_PST: [i32; 64] =
    [-20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -5,  0,  5,  5,  5,  5,  0, -5,
        0,  0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20];

static KING_MIDDLEGAME: [i32; 64] =
    [-30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        20, 20,  0,  0,  0,  0, 20, 20,
        20, 30, 10,  0,  0, 10, 30, 20];

// TODO: incorporate this
static _KING_ENDGAME: [i32; 64] =
    [-50,-40,-30,-20,-20,-30,-40,-50,
        -30,-20,-10,  0,  0,-10,-20,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-30,  0,  0,  0,  0,-30,-30,
        -50,-30,-30,-30,-30,-30,-30,-50];


fn evaluate_game(test_game: &mut Game) -> f64 {
    let mut evaluation = 0;
    let mut piece_evaluation = 0;
    for piece in test_game.pieces.clone() {
        let piece_square = bit_to_onebit_index(piece.bit);

        if piece.colour == Colour::White && piece.taken == false {
            match piece.piece_type {
                PieceType::Pawn => piece_evaluation += 100 + PAWN_PST[63 - piece_square],
                PieceType::Bishop => piece_evaluation += 320 + BISHOP_PST[63 - piece_square],
                PieceType::Knight => piece_evaluation += 330 + KNIGHT_PST[63 - piece_square],
                PieceType::Rook => piece_evaluation += 500 + ROOK_PST[63 - piece_square],
                PieceType::Queen => piece_evaluation += 900 + QUEEN_PST[63 - piece_square],
                PieceType::King => piece_evaluation += 20000 + KING_MIDDLEGAME[63 - piece_square],
            }
        }
        if piece.colour == Colour::Black && piece.taken == false {
            match piece.piece_type {
                PieceType::Pawn => piece_evaluation -= 100 + PAWN_PST[piece_square],
                PieceType::Bishop => piece_evaluation -= 320 + BISHOP_PST[piece_square],
                PieceType::Knight => piece_evaluation -= 330 + KNIGHT_PST[piece_square],
                PieceType::Rook => piece_evaluation -= 500 + ROOK_PST[piece_square],
                PieceType::Queen => piece_evaluation -= 900 + QUEEN_PST[piece_square],
                PieceType::King => piece_evaluation -= 20000 + KING_MIDDLEGAME[piece_square],
            }
        }
    }

    evaluation += piece_evaluation;

    if test_game.colour_in_check == Some(Colour::White) {
        if test_game.possible_moves.len() == 0 {
            evaluation -= 10000;
        } else {
            evaluation -= 50;
        }
    }

    if test_game.colour_in_check == Some(Colour::Black) {
        if test_game.possible_moves.len() == 0 {
            evaluation += 10000;
        } else {
            evaluation += 50;
        }
    }
    //  TODO: checkmate bonus

    return evaluation as f64 / 100.0
}

fn minimax(game: &mut Game, depth: u32, maximizing_player: bool, mut alpha: f64, mut beta: f64) -> (f64, Option<Move>) {
    // Base case: If depth is 0 or game over, return the evaluation of the game
    if depth == 0 || game.possible_moves.len() == 0 {
        return (evaluate_game(game), None);
    }

    let mut best_move: Option<Move> = None;
    let mut best_evaluation: f64;

    if maximizing_player {
        best_evaluation = f64::INFINITY;
        for possible_move in &game.possible_moves {
            let mut new_game = game.clone();
            test_move(&mut new_game, *possible_move);
            new_game.possible_moves = generate_moves(&mut new_game);
            let (evaluation, _) = minimax(&mut new_game, depth - 1, false, alpha, beta);

            if evaluation < best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(*possible_move);
            }

            if best_evaluation <= beta {
                break;
            }
            alpha = alpha.min(best_evaluation);
        }

    } else {
        best_evaluation = f64::NEG_INFINITY;
        for possible_move in &game.possible_moves {
            let mut new_game = game.clone();
            test_move(&mut new_game, *possible_move);
            new_game.possible_moves = generate_moves(&mut new_game);

            let (evaluation, _) = minimax(&mut new_game, depth - 1, true, alpha, beta);

            if evaluation > best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(*possible_move);
            }

            if best_evaluation >= alpha {
                break;
            }
            beta = beta.max(best_evaluation);
        }
    }

    (best_evaluation, best_move)
}

pub fn iterative_deepening_minimax(game: &mut Game, max_depth: u32) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut best_evaluation: f64;

    for depth in 1..=max_depth {
        best_evaluation = f64::INFINITY;
        let (evaluation, best) = minimax(game, depth, true, f64::INFINITY, f64::NEG_INFINITY);
        // Store the best move if evaluation improves
        if evaluation < best_evaluation {
            best_move = best;
        }
    }

    best_move
}