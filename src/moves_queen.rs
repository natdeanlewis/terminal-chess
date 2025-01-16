use crate::moves_bishop::add_bishop_moves;
use crate::game::Game;
use crate::moves::Move;
use crate::moves_rook::{generate_rook_moves};

pub fn add_queen_moves(
    from_square: usize,
    mut possible_moves: Vec<Move>,
    squares_to_edges: [usize; 4],
    game: &Game,
) -> Vec<Move> {
    possible_moves = add_bishop_moves(from_square, possible_moves, squares_to_edges, game);
    possible_moves.extend(generate_rook_moves(from_square, game));
    possible_moves
}