use std::cmp::min;
use lazy_static::lazy_static;
use crate::game::Game;
use crate::moves::{calculate_sliding_moves, Move};
use crate::utils::{bitboard_to_indices, onebit_index_to_bit};


lazy_static! {
    static ref BISHOP_ATTACK_MASKS: [[u64; 64]; 4] = precompute_bishop_attack_masks();
}

pub fn generate_bishop_moves(from_square: usize, game: &Game) -> Vec<Move> {
    let mut possible_moves = Vec::new();

    let occupied = game.get_occupied_bitboard();
    let own_pieces = game.get_friendly_piece_bitboard();
    for (direction, attack_masks) in BISHOP_ATTACK_MASKS.iter().enumerate() {
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

fn precompute_bishop_attack_masks() -> [[u64; 64]; 4] {
    let mut masks = [[0u64; 64]; 4];

    for square in 0..64 {
        let rank = square / 8;
        let file = square % 8;

        // North-West (r + 1, f - 1)
        let mut r = rank as isize + 1;
        let mut f = file as isize - 1;
        while r < 8 && f >= 0 {
            masks[0][square] |= 1u64 << (r * 8 + f) as usize;
            r += 1;
            f -= 1;
        }

        // North-East (r + 1, f + 1)
        r = rank as isize + 1;
        f = file as isize + 1;
        while r < 8 && f < 8 {
            masks[1][square] |= 1u64 << (r * 8 + f) as usize;
            r += 1;
            f += 1;
        }

        // South-East (r - 1, f + 1)
        r = rank as isize - 1;
        f = file as isize + 1;
        while r >= 0 && f < 8 {
            masks[2][square] |= 1u64 << (r * 8 + f) as usize;
            r -= 1;
            f += 1;
        }

        // South-West (r - 1, f - 1)
        r = rank as isize - 1;
        f = file as isize - 1;
        while r >= 0 && f >= 0 {
            masks[3][square] |= 1u64 << (r * 8 + f) as usize;
            r -= 1;
            f -= 1;
        }
    }

    masks
}

// OLD:
pub(crate) fn add_bishop_moves(
    from_square: usize,
    mut possible_moves: Vec<Move>,
    squares_to_edges: [usize; 4],
    game: &Game,
) -> Vec<Move> {
    // Diagonals clockwise from North:
    possible_moves = single_direction_bishop_moves(from_square, [squares_to_edges[0], squares_to_edges[1]], 9, game, possible_moves);
    possible_moves = single_direction_bishop_moves(from_square, [squares_to_edges[1], squares_to_edges[2]], -7, game, possible_moves);
    possible_moves = single_direction_bishop_moves(from_square, [squares_to_edges[2], squares_to_edges[3]], -9, game, possible_moves);
    possible_moves = single_direction_bishop_moves(from_square, [squares_to_edges[3], squares_to_edges[0]], 7, game, possible_moves);

    possible_moves
}

fn single_direction_bishop_moves(
    from_square: usize,
    squares_to_edges: [usize; 2],
    increment: isize,
    game: &Game,
    mut possible_moves: Vec<Move>
) -> Vec<Move> {
    let mut temp = from_square as isize;
    let max_steps = min(squares_to_edges[0], squares_to_edges[1]);

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