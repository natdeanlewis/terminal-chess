use crate::game::Game;
use crate::moves::{Move};
use crate::moves_bishop::{generate_bishop_attacked_squares_excluding_own, generate_bishop_attacked_squares_including_own};
use crate::moves_rook::{generate_rook_attacked_squares_excluding_own, generate_rook_attacked_squares_including_own};
use crate::utils::bitboard_to_indices;

pub fn generate_queen_attacked_squares_including_own(from_square: usize, game: &Game, king_bit: u64) -> u64 {
    generate_bishop_attacked_squares_including_own(from_square, game, king_bit) | generate_rook_attacked_squares_including_own(from_square, game, king_bit)
}

pub fn generate_queen_attacked_squares_excluding_own(from_square: usize, game: &Game) -> u64 {
    generate_bishop_attacked_squares_excluding_own(from_square, game) | generate_rook_attacked_squares_excluding_own(from_square, game)
}

pub fn generate_queen_moves(from_square: usize, game: &Game) -> Vec<Move> {
    let mut possible_moves = Vec::new();

    let valid_moves = generate_queen_attacked_squares_excluding_own(from_square, game);

    for target_square in bitboard_to_indices(valid_moves) {
        possible_moves.push(Move {
            from_square,
            to_square: target_square,
            promotion: None,
        });
    }

    possible_moves
}