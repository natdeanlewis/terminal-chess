use crate::game::{Game};
use crate::moves::{calculate_sliding_attacked_squares_excluding_own, calculate_sliding_attacked_squares_including_own, Move};
use crate::{bit_to_onebit_index};
use crate::utils::{bitboard_to_indices};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROOK_ATTACK_MASKS: [[u64; 64]; 4] = precompute_rook_attack_masks();
}

pub fn generate_rook_attacked_squares_including_own(from_square: usize, occupied: u64) -> u64 {
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

pub fn generate_rook_pinned_ray(pinned_piece_bit: u64, game: &Game, king_bit: u64) -> u64 {
    let king_square = bit_to_onebit_index(king_bit);
    let occupied = game.get_occupied_bitboard() & !pinned_piece_bit;
    for (direction, attack_masks) in ROOK_ATTACK_MASKS.iter().enumerate() {
        // get attacks from KING excluding pinned_piece
        let moves_in_direction = calculate_sliding_attacked_squares_including_own(
            attack_masks[king_square],
            occupied,
            direction,
        );
        if pinned_piece_bit & moves_in_direction != 0 {
            return moves_in_direction
        }
    }
    return 0u64
}

pub fn generate_rook_pinned_piece(from_square: usize, game: &Game, king_bit: u64) -> u64 {
    // // remove en passant pawn from occupied bitboard so en passant discovered checks are avoided
    // let mut occupied = game.get_occupied_bitboard();
    
    // if let Some(en_passant_bit) = game.en_passant {
    //     let en_passant_pawn_bit: u64;
    //     match game.active_colour {
    //         Colour::White => en_passant_pawn_bit = en_passant_bit >> 8,
    //         Colour::Black => en_passant_pawn_bit = en_passant_bit << 8,
    //     };
    //     occupied &= !en_passant_pawn_bit;
    //     print_bitboard(occupied);
    // }
    
    let occupied = game.get_occupied_bitboard();

    for (direction, attack_masks) in ROOK_ATTACK_MASKS.iter().enumerate() {
        // get attacks from rook
        let moves_in_direction = calculate_sliding_attacked_squares_including_own(
            attack_masks[from_square],
            occupied,
            direction,
        );

        // get opposite direction rook attacks from king
        // 0 <-> 2 1 <-> 3
        let opposite_direction = (direction + 2) % 4;
        let opposite_direction_attack_mask = ROOK_ATTACK_MASKS[opposite_direction][bit_to_onebit_index(king_bit)];
        let moves_in_opposite_direction = calculate_sliding_attacked_squares_including_own(
            opposite_direction_attack_mask,
            occupied,
            opposite_direction,
        );

        let overlap = moves_in_direction & moves_in_opposite_direction;
        if overlap != 0 {
            return overlap
        }
    }
    return 0u64
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
            capture_square: Some(target_square),
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
