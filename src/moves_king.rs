use lazy_static::lazy_static;
use crate::utils::bitboard_to_indices;
use crate::game::{CastlingRights, Colour, Game, Square};
use crate::moves::{generate_pseudolegal_moves_without_castling, square_under_threat, squares_attacked_by_opponent_bitboard, Move};

lazy_static! {
    static ref KING_MOVES: [u64; 64] = precompute_king_move_bitboards();
}
pub fn generate_legal_king_moves(from_square: usize, game: &Game) -> Vec<Move> {
    let mut possible_moves = Vec::new();

    let pseudolegal_moves = generate_king_attacked_squares(from_square, game);

    let opponent_colour = match game.active_colour {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
    };
    let squares_attacked_by_opponent = squares_attacked_by_opponent_bitboard(game, opponent_colour);

    // Don't move the king into check
    let valid_moves = pseudolegal_moves & !squares_attacked_by_opponent;

    for target_square in bitboard_to_indices(valid_moves) {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: target_square,
            promotion: None,
        });
    }
    possible_moves
}

pub fn generate_king_attacked_squares(from_square: usize, game: &Game) -> u64 {
    let king_moves = KING_MOVES[from_square];

    let occupied_by_friends = game.get_friendly_piece_bitboard();
    let valid_moves = king_moves & !occupied_by_friends;

    valid_moves
}

fn precompute_king_move_bitboards() -> [u64; 64] {
    let mut king_moves = [0u64; 64];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;

        let mut moves = 0u64;

        for (dr, df) in &[
            (-1, 0), (1, 0), (0, -1), (0, 1), // Orthogonal
            (-1, -1), (-1, 1), (1, -1), (1, 1), // Diagonal
        ] {
            let new_rank = rank as isize + dr;
            let new_file = file as isize + df;

            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let target_square = (new_rank * 8 + new_file) as usize;
                moves |= 1u64 << target_square;
            }
        }

        king_moves[square] = moves;
    }

    king_moves
}


pub fn add_castle_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
    let mut test_game = game.clone();
    match game.active_colour {
        Colour::Black => test_game.active_colour = Colour::White,
        Colour::White => test_game.active_colour = Colour::Black,
    }

    let opponent_moves = generate_pseudolegal_moves_without_castling(&mut test_game);
    if game.active_colour == Colour::White {
        if game.castling_rights.contains(CastlingRights::WHITEKINGSIDE) {
            if (5..7).all(|i| game.squares[i] == Square::Empty) && (4..7).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square + 2,
                    promotion: None,
                });
            }
        }
        if game.castling_rights.contains(CastlingRights::WHITEQUEENSIDE) {
            if (1..4).all(|i| game.squares[i] == Square::Empty) && (2..5).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square - 2,
                    promotion: None,
                });
            }
        }
    } else {
        if game.castling_rights.contains(CastlingRights::BLACKKINGSIDE) {
            if (61..63).all(|i| game.squares[i] == Square::Empty) && (60..63).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square + 2,
                    promotion: None,
                });
            }        }
        if game.castling_rights.contains(CastlingRights::BLACKQUEENSIDE) {
            if (57..60).all(|i| game.squares[i] == Square::Empty) && (58..61).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square - 2,
                    promotion: None,
                });
            }
        }
    }

    possible_moves
}