use crate::game::{Colour, Game, Piece, PieceType};
use crate::moves::{generate_moves, make_move, Move};
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


fn evaluate_game(test_game: &mut Game, maximizing_colour: Colour) -> f64 {
    let mut evaluation = 0;
    for piece in test_game.pieces.clone() {

        if piece.taken == false {
            if piece.colour == maximizing_colour {
                evaluation += piece_evaluation(&piece);
            } else {
                evaluation -= piece_evaluation(&piece);
            }
        }
    }

    let mut check_evaluation;
    if let Some(colour_in_check) = test_game.colour_in_check {
        if test_game.possible_moves.len() == 0 {
            // Checkmate
            check_evaluation = 10000;
        } else {
            // Check
            check_evaluation = 50;
        }
        if colour_in_check == maximizing_colour {
            evaluation -= check_evaluation
        } else {
            evaluation += check_evaluation
        }
    }

    return evaluation as f64 / 100.0
}

fn piece_evaluation(piece: &Piece) -> i32 {
    let mut piece_square = bit_to_onebit_index(piece.bit);

    if piece.colour == Colour::White {
        piece_square = 63 - piece_square;
    }

    match piece.piece_type {
        PieceType::Pawn => return 100 + PAWN_PST[piece_square],
        PieceType::Bishop => return 320 + BISHOP_PST[piece_square],
        PieceType::Knight => return 330 + KNIGHT_PST[piece_square],
        PieceType::Rook => return 500 + ROOK_PST[piece_square],
        PieceType::Queen => return 900 + QUEEN_PST[piece_square],
        PieceType::King =>  return 20000 + KING_MIDDLEGAME[piece_square],
    }
}

fn minimax(game: &mut Game, depth: u32, maximizing_player: bool, maximizing_colour: Colour, mut alpha: f64, mut beta: f64) -> (f64, Option<Move>) {
    // Base case: If depth is 0 or game over, return the evaluation of the game
    if depth == 0 || game.possible_moves.len() == 0 {
        return (evaluate_game(game, maximizing_colour), None);
    }

    let mut best_move: Option<Move> = None;
    let mut best_evaluation: f64;

    if maximizing_player {
        best_evaluation = f64::NEG_INFINITY;
        for possible_move in &game.possible_moves {
            let mut new_game = game.clone();
            make_move(&mut new_game, *possible_move);
            new_game.possible_moves = generate_moves(&mut new_game);

            let (evaluation, _) = minimax(&mut new_game, depth - 1, false, maximizing_colour, alpha, beta);

            if evaluation > best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(*possible_move);
            }

            if best_evaluation >= beta {
                break;
            }
            alpha = alpha.max(best_evaluation);
        }

    } else {
        best_evaluation = f64::INFINITY;
        for possible_move in &game.possible_moves {
            let mut new_game = game.clone();
            make_move(&mut new_game, *possible_move);
            new_game.possible_moves = generate_moves(&mut new_game);

            let (evaluation, _) = minimax(&mut new_game, depth - 1, true, maximizing_colour, alpha, beta);

            if evaluation < best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(*possible_move);
            }

            if best_evaluation <= alpha {
                break;
            }
            beta = beta.min(best_evaluation);
        }
    }

    (best_evaluation, best_move)
}

pub fn iterative_deepening_minimax(game: &mut Game, max_depth: u32) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut best_evaluation: f64;

    for depth in 1..=max_depth {
        best_evaluation = f64::NEG_INFINITY;
        let (evaluation, best) = minimax(game, depth, true, game.active_colour, f64::NEG_INFINITY, f64::INFINITY);
        // Store the best move if evaluation improves
        if evaluation > best_evaluation {
            best_move = best;
        }
    }

    best_move
}