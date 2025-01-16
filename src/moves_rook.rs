use crate::game::Game;
use crate::moves::{calculate_sliding_moves, Move};
use crate::utils::{bitboard_to_indices, onebit_index_to_bit};
use lazy_static::lazy_static;

lazy_static! {
    static ref ROOK_ATTACK_MASKS: [[u64; 64]; 4] = precompute_rook_attack_masks();
}

pub fn generate_rook_moves(from_square: usize, game: &Game) -> Vec<Move> {
    let mut possible_moves = Vec::new();

    let occupied = game.get_occupied_bitboard();
    let own_pieces = game.get_friendly_piece_bitboard();
    for (direction, attack_masks) in ROOK_ATTACK_MASKS.iter().enumerate() {
        let moves_in_direction = calculate_sliding_moves(
            attack_masks[from_square],
            occupied,
            direction,
            own_pieces
        );

        for target_square in bitboard_to_indices(moves_in_direction) {
            possible_moves.push(Move {
                from_square,
                to_square: target_square,
                promotion: None,
            });
        }
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

// OLD:
pub fn add_rook_moves(
    from_square: usize,
    mut possible_moves: Vec<Move>,
    squares_to_edges: [usize; 4],
    game: &Game,
) -> Vec<Move> {
    // Diagonals clockwise from North:
    possible_moves = single_direction_rook_moves(from_square, squares_to_edges[0], 8, game, possible_moves);
    possible_moves = single_direction_rook_moves(from_square, squares_to_edges[1], 1, game, possible_moves);
    possible_moves = single_direction_rook_moves(from_square, squares_to_edges[2], -8, game, possible_moves);
    possible_moves = single_direction_rook_moves(from_square, squares_to_edges[3], -1, game, possible_moves);

    possible_moves
}

fn single_direction_rook_moves(
    from_square: usize,
    squares_to_edge: usize,
    increment: isize,
    game: &Game,
    mut possible_moves: Vec<Move>
) -> Vec<Move> {
    let mut temp = from_square as isize;
    let max_steps = squares_to_edge;

    for _i in 0..max_steps {
        temp += increment;
        let temp_bit = onebit_index_to_bit(temp as usize);

        if let Some(temp_piece) = game.pieces.iter().find(|p| p.taken == false && p.bit == temp_bit) {
            if temp_piece.colour != game.active_colour {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: temp as usize,
                    promotion: None,
                });
            }
            break;
        }

        possible_moves.push(Move {
            from_square,
            to_square: temp as usize,
            promotion: None,
        });
    }

    possible_moves
}
