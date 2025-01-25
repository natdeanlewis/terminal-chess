use crate::game::{Colour, Game, Piece, PieceType};
use crate::moves::{generate_capture_moves, generate_moves, make_move, unmake_move, Move};
use crate::moves_pawn::generate_pawn_attacked_squares_including_own;
use crate::utils::{bit_to_onebit_index, onebit_index_to_bit};

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

static KING_ENDGAME: [i32; 64] =
    [-50,-40,-30,-20,-20,-30,-40,-50,
        -30,-20,-10,  0,  0,-10,-20,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-30,  0,  0,  0,  0,-30,-30,
        -50,-30,-30,-30,-30,-30,-30,-50];


fn evaluate_game(game: &mut Game) -> f64 {
    let mut evaluation = 0;
    let check_evaluation;
    let possible_moves = generate_moves(game);
    if let Some(colour_in_check) = game.colour_in_check {
        if possible_moves.len() == 0 {
            // Checkmate
            check_evaluation = 10000;
        } else {
            // Check
            check_evaluation = 50;
        }
        if colour_in_check == game.active_colour {
            evaluation -= check_evaluation
        } else {
            evaluation += check_evaluation
        }
    } else if possible_moves.len() == 0 {
        // Draw
        return 0f64
    }

    let pieces_remaining = game.pieces.iter_mut().filter(|p| !p.taken);
    let is_endgame = pieces_remaining.count() < 5;
    for piece in &game.pieces {
        if !piece.taken {
            if piece.colour == game.active_colour {
                evaluation += piece_evaluation(&piece, is_endgame);
            } else {
                evaluation -= piece_evaluation(&piece, is_endgame);
            }
        }
    }

    return evaluation as f64 / 100.0
}

fn king_positional_eval(piece_square: usize, is_endgame: bool) -> i32 {
    if is_endgame {
        return KING_ENDGAME[piece_square]
    } else {
        return KING_MIDDLEGAME[piece_square]
    }
}

fn piece_evaluation(piece: &Piece, is_endgame: bool) -> i32 {
    let mut piece_square = bit_to_onebit_index(piece.bit);

    if piece.colour == Colour::White {
        piece_square = 63 - piece_square;
    }

    match piece.piece_type {
        PieceType::Pawn => return piece_value(PieceType::Pawn) + PAWN_PST[piece_square],
        PieceType::Bishop => return piece_value(PieceType::Bishop) + BISHOP_PST[piece_square],
        PieceType::Knight => return piece_value(PieceType::Knight) + KNIGHT_PST[piece_square],
        PieceType::Rook => return piece_value(PieceType::Rook) + ROOK_PST[piece_square],
        PieceType::Queen => return piece_value(PieceType::Queen) + QUEEN_PST[piece_square],
        PieceType::King =>  return piece_value(PieceType::King) + king_positional_eval(piece_square, is_endgame),
    }
}

fn piece_value(piece_type: PieceType) -> i32 {
    return match piece_type {
        PieceType::Pawn => 100,
        PieceType::Bishop => 320,
        PieceType::Knight => 330,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 20_000,
    }
}
fn order_moves(moves: Vec<Move>, game: &mut Game) -> Vec<Move> {
    let mut ordered_moves: Vec<(Move, i32)> = Vec::new();  // Vector to store moves and their respective scores
    let mut opponent_pawn_attacked_squares = 0u64;
    let opponent_colour = match game.active_colour {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
    };

    for piece in &game.pieces {
        if piece.colour == opponent_colour && piece.taken == false {
            let from_square = bit_to_onebit_index(piece.bit);
            match piece.piece_type {
                PieceType::Pawn => {
                    opponent_pawn_attacked_squares |= generate_pawn_attacked_squares_including_own(from_square, opponent_colour);
                },
                _ => ()
            }
        }
    }

    for unordered_move in moves.iter() {
        let mut move_score_guess = 0;
        if let Some(moving_piece) = game.pieces.iter().find(|&p| p.bit == onebit_index_to_bit(unordered_move.from_square) && !p.taken) {
            if let Some(captured_piece) = game.pieces.iter().find(|&p| p.bit == onebit_index_to_bit(unordered_move.to_square) && !p.taken) {
                move_score_guess = 10 * piece_value(captured_piece.piece_type) - piece_value(moving_piece.piece_type);
            }

            if let Some(promotion_piece_type) = unordered_move.promotion {
                move_score_guess += piece_value(promotion_piece_type);
            }

            if opponent_pawn_attacked_squares & onebit_index_to_bit(unordered_move.to_square) != 0 {
                move_score_guess -= piece_value(moving_piece.piece_type);
            }

            ordered_moves.push((*unordered_move, move_score_guess));
        }
    }
    ordered_moves.sort_by(|a, b| b.1.cmp(&a.1));
    return ordered_moves.into_iter().map(|(m, _)| m).collect();
}

fn search(game: &mut Game, depth: u32, mut alpha: f64, mut beta: f64) -> (f64, Option<Move>) {
    let mut possible_moves = generate_moves(game);

    // Base case: If depth is 0 or game over, return the evaluation of the game
    if depth == 0 || possible_moves.len() == 0 {
        return (search_all_captures(game, alpha, beta), None);
    }

    let mut best_move: Option<Move> = None;
    let mut best_evaluation = f64::NEG_INFINITY;

    possible_moves = order_moves(possible_moves, game);

    for i in 0..possible_moves.len() {
        let possible_move = possible_moves[i];
        let move_to_unmake = make_move(game, possible_move);
        let (negative_evaluation, _) = search(game, depth - 1, -beta, -alpha);
        let evaluation = -1f64 * negative_evaluation;
        unmake_move(game, move_to_unmake);

        if evaluation > best_evaluation {
            best_evaluation = evaluation;
            best_move = Some(possible_move);
        }

        if best_evaluation >= beta {
            break;
        }
        alpha = alpha.max(best_evaluation);
    }
    (best_evaluation, best_move)
}

fn search_all_captures(game: &mut Game, mut alpha: f64, mut beta: f64) -> f64 {
    let evaluation = evaluate_game(game);
    if evaluation >= beta {
        return beta;
    }
    alpha = alpha.max(evaluation);

    let mut capture_moves = generate_capture_moves(game);
    capture_moves = order_moves(capture_moves, game);

    for i in 0..capture_moves.len() {
        let possible_move = capture_moves[i];
        let move_to_unmake = make_move(game, possible_move);
        let negative_evaluation = search_all_captures(game, -beta, -alpha);
        let evaluation = -1f64 * negative_evaluation;
        unmake_move(game, move_to_unmake);

        if evaluation >= beta {
            break;
        }
        alpha = alpha.max(evaluation);
    }
    evaluation
}

pub fn iterative_deepening_minimax(game: &mut Game, max_depth: u32) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut best_evaluation: f64;
    
    for depth in 1..=max_depth {
        best_evaluation = f64::NEG_INFINITY;
        let (evaluation, best) = search(game, depth, f64::NEG_INFINITY, f64::INFINITY);
        // Store the best move if evaluation improves
        if evaluation > best_evaluation {
            best_move = best;
        }
    }

    best_move
}