use std::io;
use std::io::Write;
use crate::bishop_moves::add_bishop_moves;
use crate::game::{CastlingRights, Game, PieceType, Square};
use crate::utils::{bit_to_onebit_index, get_piece_index, onebit_index_to_bit, print_board};
use crate::Colour;
use crate::king_moves::{add_castle_moves, add_king_moves};
use crate::knight_moves::add_knight_moves;
use crate::pawn_moves::add_pawn_moves;
use crate::queen_moves::add_queen_moves;
use crate::rook_moves::add_rook_moves;

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
                    possible_moves = add_queen_moves(from_square, possible_moves, squares_to_edges, game);
                },
                PieceType::King => {
                    let squares_to_edges  = squares_to_edges(piece.bit);
                    possible_moves = add_king_moves(from_square, possible_moves, squares_to_edges, game);
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

pub fn offset_matches_row_offset(from_square: usize, offset: isize, row_offset: isize) -> bool {
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

pub fn square_under_threat(square_index: usize, opponent_moves: &Vec<Move>) -> bool {
    return opponent_moves.iter().any(|m| m.to_square == square_index)
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