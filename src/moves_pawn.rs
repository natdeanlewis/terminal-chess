use crate::game::{Colour, Game};
use crate::moves::Move;
use crate::utils::{bit_to_onebit_index, onebit_index_to_bit};

pub fn add_pawn_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
    let increment: isize;
    let start_row: usize;
    if game.active_colour == Colour::White {
        increment = 8;
        start_row = 2;
    } else {
        increment = -8;
        start_row =  7;
    }

    // One square forwards
    let mut target_square = from_square as isize + increment;
    let mut target_bit = onebit_index_to_bit(target_square as usize);
    if game.pieces.iter().all(|p| p.taken || p.bit != target_bit) {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: target_square as usize,
        });

        // Two squares forward
        if from_square / 8 + 1 == start_row {
            target_square += increment;
            target_bit = onebit_index_to_bit(target_square as usize);
            if game.pieces.iter().all(|p| p.taken || p.bit != target_bit) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: target_square as usize,
                })
            }
        }
    }

    // Left diagonal capture
    if from_square % 8 > 0 {
        let left_diagonal_target_square = from_square as isize + increment - 1;
        let left_diagonal_target_bit = onebit_index_to_bit(left_diagonal_target_square as usize);

        if let Some(_piece) = game.pieces.iter().find(|p| p.taken == false && p.bit == left_diagonal_target_bit && p.colour != game.active_colour) {
            possible_moves.push(Move {
                from_square: from_square,
                to_square: left_diagonal_target_square as usize,
            });
        }
    }

    // Right diagonal capture
    if from_square % 8 < 7 {
        let right_diagonal_target_square = from_square as isize + increment + 1;
        let right_diagonal_target_bit = onebit_index_to_bit(right_diagonal_target_square as usize);

        if let Some(_piece) = game.pieces.iter().find(|p| p.taken == false && p.bit == right_diagonal_target_bit && p.colour != game.active_colour) {
            possible_moves.push(Move {
                from_square: from_square,
                to_square: right_diagonal_target_square as usize,
            });
        }
    }

    //en passant
    match game.en_passant {
        Some(en_passant) => {
            let en_passant_onebit_index = bit_to_onebit_index(en_passant);
            // Left diagonal
            if from_square % 8 > 0 {
                let left_diagonal_target_square = from_square as isize + increment - 1;

                if left_diagonal_target_square == en_passant_onebit_index as isize {
                    possible_moves.push(Move {
                        from_square: from_square,
                        to_square: en_passant_onebit_index,
                    })
                }
            }

            // Right diagonal
            if from_square % 8 < 7 {
                let left_diagonal_target_square = from_square as isize + increment + 1;

                if left_diagonal_target_square == en_passant_onebit_index as isize {
                    possible_moves.push(Move {
                        from_square: from_square,
                        to_square: en_passant_onebit_index,
                    })
                }
            }
        }
        _ => {}
    }

    possible_moves
}
