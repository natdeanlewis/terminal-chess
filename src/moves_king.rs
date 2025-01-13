use crate::moves_bishop::add_bishop_moves;
use crate::game::{CastlingRights, Colour, Game, Square};
use crate::moves::{generate_pseudolegal_moves_without_castling, square_under_threat, Move};
use crate::moves_rook::add_rook_moves;

pub fn add_king_moves(
from_square: usize,
mut possible_moves: Vec<Move>,
squares_to_edges: [usize; 4],
game: &Game,
)
-> Vec<Move> {
    let mut king_squares_to_edges: [usize; 4] = [0; 4];
    for (i, squares_to_edge) in squares_to_edges.iter().enumerate() {
        if *squares_to_edge > 0 {
            king_squares_to_edges[i] = 1;
        }
    }
    possible_moves = add_bishop_moves(from_square, possible_moves, king_squares_to_edges, game);
    possible_moves = add_rook_moves(from_square, possible_moves, king_squares_to_edges, game);

    possible_moves
}

pub fn add_castle_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
    let mut test_game = game.clone();
    match game.active_colour {
        Colour::Black => test_game.active_colour = Colour::White,
        Colour::White => test_game.active_colour = Colour::Black,
    }

    let opponent_moves = generate_pseudolegal_moves_without_castling(&mut test_game);
    if game.active_colour == Colour::White {
        if game.castling_rights.contains(CastlingRights::WHITEKINGSIDE) {
            if (5..7).all(|i| game.squares[i] == Square::Empty) && (4..7).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square + 2,
                    promotion: None,
                });
            }
        }
        if game.castling_rights.contains(CastlingRights::WHITEQUEENSIDE) {
            if (1..4).all(|i| game.squares[i] == Square::Empty) && (2..5).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square - 2,
                    promotion: None,
                });
            }
        }
    } else {
        if game.castling_rights.contains(CastlingRights::BLACKKINGSIDE) {
            if (61..63).all(|i| game.squares[i] == Square::Empty) && (60..63).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square + 2,
                    promotion: None,
                });
            }        }
        if game.castling_rights.contains(CastlingRights::BLACKQUEENSIDE) {
            if (57..60).all(|i| game.squares[i] == Square::Empty) && (58..61).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square - 2,
                    promotion: None,
                });
            }
        }
    }

    possible_moves
}