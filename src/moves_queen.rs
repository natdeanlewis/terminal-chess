use crate::moves_bishop::generate_bishop_moves;
use crate::game::Game;
use crate::moves::Move;
use crate::moves_rook::generate_rook_moves;

pub fn add_queen_moves(
    from_square: usize,
    mut possible_moves: Vec<Move>,
    game: &Game,
) -> Vec<Move> {
    possible_moves.extend(generate_bishop_moves(from_square, game));
    possible_moves.extend(generate_rook_moves(from_square, game));
    possible_moves
}