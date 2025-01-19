use crate::game::{Game};
use crate::moves::{calculate_sliding_attacked_squares_including_own, calculate_sliding_attacked_squares_excluding_own, Move};
use crate::onebit_index_to_bit;
use crate::utils::{bitboard_to_indices};
use lazy_static::lazy_static;

lazy_static! {
    static ref ROOK_ATTACK_MASKS: [[u64; 64]; 4] = precompute_rook_attack_masks();
}

pub fn generate_rook_attacked_squares_including_own(from_square: usize, game: &Game, occupied: u64) -> u64 {
    let mut attacked_squares = 0u64;
    for (direction, attack_masks) in ROOK_ATTACK_MASKS.iter().enumerate() {
        let moves_in_direction = calculate_sliding_attacked_squares_including_own(
            attack_masks[from_square],
            occupied,
            direction,
        );

        attacked_squares |= moves_in_direction;
    }

    attacked_squares
}

pub fn generate_rook_absolute_pins(from_square: usize, game: &Game, king_bit: u64) -> Vec<u64> {
    let mut absolute_pins = vec![];
    // Ignore non-king pieces from the occupied squares to generate pins
    let occupied = king_bit;
    for (direction, attack_masks) in ROOK_ATTACK_MASKS.iter().enumerate() {
        let moves_in_direction = calculate_sliding_attacked_squares_including_own(
            attack_masks[from_square],
            occupied,
            direction,
        );

        if moves_in_direction & king_bit != 0 {
            // include pinning piece so taking it is a valid move to solve the pin
            absolute_pins.push(moves_in_direction | onebit_index_to_bit(from_square));
        }
    }

    absolute_pins
}


pub fn generate_rook_attacked_squares_excluding_own(from_square: usize, game: &Game) -> u64 {
    let mut attacked_squares = 0u64;

    let occupied = game.get_occupied_bitboard();
    let own_pieces = game.get_friendly_piece_bitboard();
    for (direction, attack_masks) in ROOK_ATTACK_MASKS.iter().enumerate() {
        let moves_in_direction = calculate_sliding_attacked_squares_excluding_own(
            attack_masks[from_square],
            occupied,
            direction,
            own_pieces
        );

        attacked_squares |= moves_in_direction;
    }

    attacked_squares
}

pub fn generate_rook_moves(from_square: usize, game: &Game) -> Vec<Move> {
    let mut possible_moves = Vec::new();

    let valid_moves = generate_rook_attacked_squares_excluding_own(from_square, game);

    for target_square in bitboard_to_indices(valid_moves) {
        possible_moves.push(Move {
            from_square,
            to_square: target_square,
            promotion: None,
        });
    }

    possible_moves
}


fn precompute_rook_attack_masks() -> [[u64; 64]; 4] {
    let mut masks = [[0u64; 64]; 4];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;

        // North
        for r in (rank + 1)..8 {
            masks[0][square] |= 1u64 << (r * 8 + file);
        }
        // East
        for f in (file + 1)..8 {
            masks[1][square] |= 1u64 << (rank * 8 + f);
        }
        // South
        for r in (0..rank).rev() {
            masks[2][square] |= 1u64 << (r * 8 + file);
        }
        // West
        for f in (0..file).rev() {
            masks[3][square] |= 1u64 << (rank * 8 + f);
        }
    }

    masks
}
