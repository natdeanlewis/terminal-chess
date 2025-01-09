use crate::game::{Game, Piece, PieceType};
use crate::utils::bit_to_onebit_index;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub(crate) from_square: usize,
    pub(crate) to_square: usize,
}

fn squares_to_edges(bit: u64) -> [usize; 4] {
    let onebit_index = bit_to_onebit_index(bit);
    let column_num = onebit_index % 8 + 1;
    let row_num = onebit_index / 8 + 1;
    [8 - row_num, 8 - column_num, row_num - 1, column_num - 1]
}

pub fn generate_moves(game: &mut Game) -> Vec<Move> {
    let mut possible_moves: Vec<Move> = Vec::new();
    for piece in &game.pieces {
        if piece.colour == game.active_colour {
            let from_square = bit_to_onebit_index(piece.bit);
            let squares_to_edges  = squares_to_edges(piece.bit);
            match piece.piece_type {
                PieceType::Knight => {
                    possible_moves = add_knight_moves(piece, from_square, possible_moves, squares_to_edges);
                }
                _ => {}
            }
        }
    }

    possible_moves
}

fn add_knight_moves(piece: &Piece, from_square: usize, mut possible_moves: Vec<Move>, squares_to_edges: [usize; 4]) -> Vec<Move> {
    if squares_to_edges[0] >= 2 && squares_to_edges[1] >= 1 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square + 17
        });
    }
    if squares_to_edges[0] >= 1 && squares_to_edges[1] >= 2 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square + 10
        });
    }
    if squares_to_edges[2] >= 1 && squares_to_edges[1] >= 2 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square - 6
        });
    }
    if squares_to_edges[2] >= 2 && squares_to_edges[1] >= 1 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square - 15
        });
    }
    if squares_to_edges[2] >= 2 && squares_to_edges[3] >= 1 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square - 17
        });
    }
    if squares_to_edges[2] >= 1 && squares_to_edges[3] >= 2 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square - 10
        });
    }
    if squares_to_edges[0] >= 1 && squares_to_edges[3] >= 2 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square + 6
        });
    }
    if squares_to_edges[0] >= 2 && squares_to_edges[3] >= 1 {
        possible_moves.push(Move {
            from_square: from_square,
            to_square: from_square + 15
        });
    }

    possible_moves
}
