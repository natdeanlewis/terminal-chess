use crate::game::{Game};
use crate::moves::{Move};
use lazy_static::lazy_static;
use crate::utils::bitboard_to_indices;

lazy_static! {
    static ref KNIGHT_MOVES: [u64; 64] = precompute_knight_move_bitboards();
}

pub fn generate_knight_moves(from_square: usize, game: &Game) -> Vec<Move> {

    let mut possible_moves = Vec::new();

    let valid_moves = generate_knight_attacked_squares_excluding_own(from_square, game);

    for target_square in bitboard_to_indices(valid_moves) {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: target_square,
            promotion: None,
            capture_square: None,
        });
    }

    possible_moves
}

pub fn generate_knight_attacked_squares_including_own(from_square: usize) -> u64 {
    KNIGHT_MOVES[from_square]
}

pub fn generate_knight_attacked_squares_excluding_own(from_square: usize, game: &Game) -> u64 {
    let knight_moves = generate_knight_attacked_squares_including_own(from_square);

    let occupied_by_friends = game.get_friendly_piece_bitboard();
    let valid_moves = knight_moves & !occupied_by_friends;

    valid_moves
}

fn precompute_knight_move_bitboards() -> [u64; 64] {
    let mut knight_moves = [0u64; 64];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;

        let mut moves = 0u64;

        for (dr, df) in &[
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2),
        ] {
            let new_rank = rank as isize + dr;
            let new_file = file as isize + df;

            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let target_square = (new_rank * 8 + new_file) as usize;
                moves |= 1u64 << target_square;
            }
        }

        knight_moves[square] = moves;
    }

    knight_moves
}
