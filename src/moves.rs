use std::cmp::min;
use std::io;
use std::io::Write;
use crate::game::{CastlingRights, Game, PieceType, Square};
use crate::utils::{bit_to_onebit_index, get_piece_index, onebit_index_to_bit, print_board};
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

pub fn generate_pseudolegal_moves_without_castling(game: &mut Game) -> Vec<Move> {
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
                }
            }
        }
    }

    possible_moves
}

fn generate_pseudolegal_moves(game: &mut Game) -> Vec<Move> {
    let mut possible_moves = generate_pseudolegal_moves_without_castling(game);

    for piece in &game.pieces {
        if piece.colour == game.active_colour && piece.piece_type == PieceType::King {
            let from_square = bit_to_onebit_index(piece.bit);
            match piece.piece_type {
                PieceType::King => {
                    possible_moves = add_castle_moves(from_square, possible_moves, game);
                }
                _ => ()
            }
        }
    }

    possible_moves
}

pub fn generate_moves(game: &mut Game) -> Vec<Move> {
    let possible_moves = generate_pseudolegal_moves(game);

    // Only include moves that don't result in a check on the active colour
    let mut new_possible_moves = vec![];

    for possible_move in possible_moves {

        let mut new_game = game.clone();

        test_move(&mut new_game, possible_move);

        if let Some(king) = new_game.pieces.iter().find(|p| p.piece_type == PieceType::King && p.colour != new_game.active_colour) {
            let king_square = bit_to_onebit_index(king.bit);

            if !inactive_colour_in_check(&mut new_game, king_square) {
                new_possible_moves.push(possible_move);
            }
        }
    }

    new_possible_moves
}

pub fn inactive_colour_in_check(game: &mut Game, king_square: usize) -> bool {
    let next_possible_moves= generate_pseudolegal_moves(game);
    if next_possible_moves.iter().any(|m| m.to_square == king_square) {
        return true;
    }
    false
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

fn square_under_threat(square_index: usize, opponent_moves: &Vec<Move>) -> bool {
    return opponent_moves.iter().any(|m| m.to_square == square_index)
}

fn add_castle_moves(from_square: usize, mut possible_moves: Vec<Move>, game: &Game) -> Vec<Move> {
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
                });
            }
        }
        if game.castling_rights.contains(CastlingRights::WHITEQUEENSIDE) {
            if (1..4).all(|i| game.squares[i] == Square::Empty) && (2..5).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square - 2,
                });
            }
        }
    } else {
        if game.castling_rights.contains(CastlingRights::BLACKKINGSIDE) {
            if (61..63).all(|i| game.squares[i] == Square::Empty) && (60..63).all(|i| !square_under_threat(i, &opponent_moves)) {
                possible_moves.push(Move {
                    from_square: from_square,
                    to_square: from_square + 2,
                });
            }        }
        if game.castling_rights.contains(CastlingRights::BLACKQUEENSIDE) {
            if (57..60).all(|i| game.squares[i] == Square::Empty) && (58..61).all(|i| !square_under_threat(i, &opponent_moves)) {
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

pub fn test_move(game: &mut Game, move_to_make: Move) {
    let start_bit = onebit_index_to_bit(move_to_make.from_square);
    let end_bit = onebit_index_to_bit(move_to_make.to_square);

    if let Some(start_piece_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == start_bit && p.colour == game.active_colour) {
        make_pawn_promotion_auto_queen(game, move_to_make, start_piece_index);
        make_non_pawn_promotion_move(game, move_to_make, start_piece_index, end_bit);
    }
}

pub fn make_move(game: &mut Game, move_to_make: Move) {
    let start_bit = onebit_index_to_bit(move_to_make.from_square);
    let end_bit = onebit_index_to_bit(move_to_make.to_square);

    if let Some(start_piece_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == start_bit && p.colour == game.active_colour) {
        make_pawn_promotion_user_choice(game, move_to_make, start_piece_index);
        make_non_pawn_promotion_move(game, move_to_make, start_piece_index, end_bit);
    }
}

fn make_non_pawn_promotion_move(game: &mut Game, move_to_make: Move, start_piece_index: usize, end_bit: u64) {
    // Castling
    if game.pieces[start_piece_index].piece_type == PieceType::King {
        // Remove queen and king side castling rights
        match game.active_colour {
            Colour::White => {
                game.castling_rights.remove(CastlingRights::WHITEKINGSIDE);
                game.castling_rights.remove(CastlingRights::WHITEQUEENSIDE);
            }
            Colour::Black => {
                game.castling_rights.remove(CastlingRights::BLACKKINGSIDE);
                game.castling_rights.remove(CastlingRights::BLACKQUEENSIDE);
            }
        }
        if (move_to_make.to_square as isize - move_to_make.from_square as isize).abs() == 2 {
            let king_side_rook_square;
            let queen_side_rook_square;
            if game.active_colour == Colour::White {
                king_side_rook_square = 7;
                queen_side_rook_square = 0;
            } else {
                king_side_rook_square = 63;
                queen_side_rook_square = 56;
            }

            if move_to_make.to_square > move_to_make.from_square {
                // King side rook
                if let Some(rook) = game.pieces.iter_mut().find(|p| p.bit == onebit_index_to_bit(king_side_rook_square)) {
                    rook.bit = onebit_index_to_bit(move_to_make.from_square + 1);
                }
                if let Some(rook_piece_index) = get_piece_index(&game.squares[move_to_make.from_square + 3]) {
                    game.squares[move_to_make.from_square + 1] = Square::Occupied(rook_piece_index);
                    game.squares[move_to_make.from_square + 3] = Square::Empty;
                }
            } else {
                // Queen side rook
                if let Some(rook) = game.pieces.iter_mut().find(|p| p.bit == onebit_index_to_bit(queen_side_rook_square)) {
                    rook.bit = onebit_index_to_bit(move_to_make.from_square - 1);
                }
                if let Some(rook_piece_index) = get_piece_index(&game.squares[move_to_make.from_square - 4]) {
                    game.squares[move_to_make.from_square - 1] = Square::Occupied(rook_piece_index);
                    game.squares[move_to_make.from_square - 4] = Square::Empty;
                }
            }
        }
    }
    if game.pieces[start_piece_index].piece_type == PieceType::Rook {
        //Remove this rook's side castling rights
        match start_piece_index {
            0 => {
                game.castling_rights.remove(CastlingRights::WHITEQUEENSIDE);
            }
            7 => {
                game.castling_rights.remove(CastlingRights::WHITEKINGSIDE);
            }
            56 => {
                game.castling_rights.remove(CastlingRights::BLACKQUEENSIDE);
            }
            63 => {
                game.castling_rights.remove(CastlingRights::BLACKKINGSIDE);
            }
            _ => {}
        }

    }

    // En passant capture
    match game.en_passant {
        Some(en_passant_bit) => {
            if end_bit == en_passant_bit && game.pieces[start_piece_index].piece_type == PieceType::Pawn {
                let captured_piece_square;
                if game.active_colour == Colour::White {
                    captured_piece_square = move_to_make.to_square - 8;
                } else {
                    captured_piece_square = move_to_make.to_square + 8;
                }
                let captured_piece_bit = onebit_index_to_bit(captured_piece_square);
                if let Some(captured_piece_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == captured_piece_bit) {
                    game.pieces[captured_piece_index].taken = true;
                    game.squares[captured_piece_square] = Square::Empty;
                }
            }
        }
        _ => {}
    }

    // Standard capture
    if let Some(target_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == end_bit) {
        game.pieces[target_index].taken = true;
        if game.pieces[target_index].piece_type == PieceType::Rook {
            // Remove this rook's side castling rights
            let captured_piece_square = bit_to_onebit_index(game.pieces[target_index].bit);
            match captured_piece_square {
                0 => {
                    game.castling_rights.remove(CastlingRights::WHITEQUEENSIDE);
                }
                7 => {
                    game.castling_rights.remove(CastlingRights::WHITEKINGSIDE);
                }
                56 => {
                    game.castling_rights.remove(CastlingRights::BLACKQUEENSIDE);
                }
                63 => {
                    game.castling_rights.remove(CastlingRights::BLACKKINGSIDE);
                }
                _ => {}
            }
        }
    }

    let piece_index = get_piece_index(&game.squares[move_to_make.from_square]);
    game.squares[move_to_make.to_square] = Square::Occupied(piece_index.unwrap());
    game.squares[move_to_make.from_square] = Square::Empty;
    game.pieces[start_piece_index].bit = end_bit;

    if game.pieces[start_piece_index].piece_type == PieceType::Pawn && (move_to_make.to_square as isize - move_to_make.from_square as isize).abs() == 16 {
        let en_passant_square = (move_to_make.from_square + move_to_make.to_square) / 2;
        game.en_passant = Some(onebit_index_to_bit(en_passant_square));
    } else {
        game.en_passant = None;
    }

    let inactive_colour = match game.active_colour {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
    };

    if let Some(king) = game.pieces.iter().find(|p| p.piece_type == PieceType::King && p.colour != game.active_colour) {
        let king_square = bit_to_onebit_index(king.bit);

        if inactive_colour_in_check(game, king_square) {
            game.colour_in_check = Some(inactive_colour);
        } else {
            game.colour_in_check = None
        }
    }

    if game.active_colour == Colour::Black {
        game.fullmove_number += 1;
    }

    game.active_colour = inactive_colour;
}


fn make_pawn_promotion_user_choice(game: &mut Game, move_to_make: Move, start_piece_index: usize) {
    // Pawn promotion
    let promotion_row;
    if game.active_colour == Colour::White {
        promotion_row = 7;
    } else {
        promotion_row = 0;
    }
    if game.pieces[start_piece_index].piece_type == PieceType::Pawn && move_to_make.to_square / 8 == promotion_row {
        // TODO: add options to move gen for CPU?
        if game.active_colour == Colour::Black {
            game.pieces[start_piece_index].piece_type = PieceType::Queen;
        } else {
            let mut promotion_piece_type: Option<PieceType> = None;
            while promotion_piece_type == None {
                print_board(&game);
                print!("Piece to promote to (Q for Queen, R for Rook, N for Knight, B for Bishop): ");
                io::stdout().flush().unwrap();
                let mut promotion_input = String::new();
                io::stdin().read_line(&mut promotion_input).unwrap();
                promotion_input = promotion_input.trim().to_string();
                if promotion_input != "" {
                    promotion_piece_type = match promotion_input.chars().next().unwrap().to_ascii_lowercase() {
                        'q' => Some(PieceType::Queen),
                        'r' => Some(PieceType::Rook),
                        'n' => Some(PieceType::Knight),
                        'b' => Some(PieceType::Bishop),
                        _ => None,
                    };
                }
            }
            game.pieces[start_piece_index].piece_type = promotion_piece_type.expect("!");
        }
    }
}

fn make_pawn_promotion_auto_queen(game: &mut Game, move_to_make: Move, start_piece_index: usize) {
    let promotion_row;
    if game.active_colour == Colour::White {
        promotion_row = 7;
    } else {
        promotion_row = 0;
    }
    if game.pieces[start_piece_index].piece_type == PieceType::Pawn && move_to_make.to_square / 8 == promotion_row {
        game.pieces[start_piece_index].piece_type = PieceType::Queen;
    }
}