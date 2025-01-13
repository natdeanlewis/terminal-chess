use crate::game::Game;
use crate::moves::{offset_matches_row_offset, Move};
use crate::utils::onebit_index_to_bit;

pub fn add_knight_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
    // Knight moves clockwise from North:
    possible_moves = single_direction_knight_move(from_square, 17, 2, possible_moves, game);
    possible_moves = single_direction_knight_move(from_square, 10, 1, possible_moves, game);
    possible_moves = single_direction_knight_move(from_square, -6, -1, possible_moves, game);
    possible_moves = single_direction_knight_move(from_square, -15, -2, possible_moves, game);
    possible_moves = single_direction_knight_move(from_square, -17, -2, possible_moves, game);
    possible_moves = single_direction_knight_move(from_square, -10, -1, possible_moves, game);
    possible_moves = single_direction_knight_move(from_square, 6, 1, possible_moves, game);
    possible_moves = single_direction_knight_move(from_square, 15, 2, possible_moves, game);

    possible_moves
}

fn single_direction_knight_move(from_square: usize, offset: isize, row_offset: isize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move>{
    if offset_matches_row_offset(from_square, offset, row_offset) {
        let target_index = from_square as isize + offset;
        let target_bit = onebit_index_to_bit(target_index as usize);

        if game.pieces.iter().all(|p| p.taken || p.bit != target_bit || p.colour != game.active_colour) {
            possible_moves.push(Move {
                from_square: from_square,
                to_square: target_index as usize,
                promotion: None,
            });
        }
    }

    possible_moves
}