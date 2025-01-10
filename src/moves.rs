use std::cmp::min;

use crate::game::{CastlingRights, Game, PieceType, Square};
use crate::utils::{bit_to_onebit_index, coords_to_onebit_index, onebit_index_to_bit};
use crate::Colour;

#[derive(Debug, PartialEq, Clone, Copy)]
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
        if piece.colour == game.active_colour && piece.taken == false {
            let from_square = bit_to_onebit_index(piece.bit);
            match piece.piece_type {
                PieceType::Pawn => {
                    possible_moves = add_pawn_moves(from_square, possible_moves, game);
                }
                PieceType::Knight => {
                    possible_moves = add_knight_moves(from_square, possible_moves, game);
                },
                PieceType::Bishop => {
                    let squares_to_edges  = squares_to_edges(piece.bit);
                    possible_moves = add_bishop_moves(from_square, possible_moves, squares_to_edges, game);
                },
                PieceType::Rook => {
                    let squares_to_edges  = squares_to_edges(piece.bit);
                    possible_moves = add_rook_moves(from_square, possible_moves, squares_to_edges, game);
                },
                PieceType::Queen =>  {
                    let squares_to_edges  = squares_to_edges(piece.bit);
                    possible_moves = add_bishop_moves(from_square, possible_moves, squares_to_edges, game);
                    possible_moves = add_rook_moves(from_square, possible_moves, squares_to_edges, game);
                },
                PieceType::King => {
                    let squares_to_edges  = squares_to_edges(piece.bit);
                    let mut king_squares_to_edges: [usize; 4] = [0; 4];
                    for (i, squares_to_edge) in squares_to_edges.iter().enumerate() {
                        if *squares_to_edge > 0 {
                            king_squares_to_edges[i] = 1;
                        }
                    }
                    possible_moves = add_bishop_moves(from_square, possible_moves, king_squares_to_edges, game);
                    possible_moves = add_rook_moves(from_square, possible_moves, king_squares_to_edges, game);

                    possible_moves = add_castle_moves(from_square, possible_moves, game);
                }
            }
        }
    }

    possible_moves
}

fn offset_matches_row_offset(from_square: usize, offset: isize, row_offset: isize) -> bool {
    let target_square = from_square as isize + offset;
    if target_square < 0 {
        return false
    }
    let target_row = target_square / 8;
    if target_row >= 8 {
        return false
    }
    let from_row =  from_square / 8;
    return target_row - from_row as isize == row_offset;
}

fn add_castle_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
    println!("{:?}", game.castling_rights);
    if game.active_colour == Colour::White {
        if game.castling_rights.contains(CastlingRights::WHITEKINGSIDE) {
            if (5..7).all(|i| game.squares[i] == Square::Empty) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square + 2,
                });
            }
        }
        if game.castling_rights.contains(CastlingRights::WHITEQUEENSIDE) {
            if (1..4).all(|i| game.squares[i] == Square::Empty) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square - 2,
                });
            }
        }
    } else {
        if game.castling_rights.contains(CastlingRights::BLACKKINGSIDE) {
            if (61..63).all(|i| game.squares[i] == Square::Empty) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square + 2,
                });
            }        }
        if game.castling_rights.contains(CastlingRights::BLACKQUEENSIDE) {
            if (57..60).all(|i| game.squares[i] == Square::Empty) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square - 2,
                });
            }
        }
    }

    possible_moves
}
fn add_pawn_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
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

fn add_knight_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
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
                to_square: target_index as usize
            });
        }
    }

    possible_moves
}

fn add_bishop_moves(
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
                });
            }
            break;
        }
        
        possible_moves.push(Move {
            from_square,
            to_square: temp as usize,
        });
    }

    possible_moves
}

fn add_rook_moves(
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
            });
        }
        break;
    }
    
    possible_moves.push(Move {
        from_square,
        to_square: temp as usize,
    });
}

possible_moves
}