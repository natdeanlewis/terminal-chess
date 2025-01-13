use std::cmp::min;
use crate::game::Game;
use crate::moves::Move;
use crate::utils::onebit_index_to_bit;

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