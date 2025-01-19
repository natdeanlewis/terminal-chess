use crate::game::{Colour, Game, PieceType};
use crate::game::Colour::White;
use crate::moves::{calculate_sliding_attacked_squares_including_own, Move};
use crate::moves_rook::{ROOK_ATTACK_MASKS};
use crate::utils::{bit_to_onebit_index, onebit_index_to_bit};

pub fn generate_pawn_attacked_squares_including_own(from_square: usize, colour: Colour) -> u64 {
    let mut attacked_squares = 0u64;

    let increment: isize;
    let end_row_index: usize;
    if colour == Colour::White {
        increment = 8;
        end_row_index = 7;
    } else {
        increment = -8;
        end_row_index = 0;
    }

    let current_index = from_square / 8;
    if current_index != end_row_index {
        // Left diagonal capture
        if from_square % 8 > 0 {
            let left_diagonal_target_square = from_square as isize + increment - 1;
            let left_diagonal_target_bit = onebit_index_to_bit(left_diagonal_target_square as usize);

            attacked_squares |= left_diagonal_target_bit
        }

        // Right diagonal capture
        if from_square % 8 < 7 {
            let right_diagonal_target_square = from_square as isize + increment + 1;
            let right_diagonal_target_bit = onebit_index_to_bit(right_diagonal_target_square as usize);

            attacked_squares |= right_diagonal_target_bit
        }
    }
    attacked_squares
}


pub fn generate_pawn_moves(from_square: usize, game: &Game) -> Vec<Move> {
    let mut possible_moves = Vec::new();

    let increment: isize;
    let start_row_index: usize;
    let end_row_index: usize;
    let mut pawn_moves: Vec<Move> = Vec::new();
    if game.active_colour == Colour::White {
        increment = 8;
        start_row_index = 1;
        end_row_index = 7;
    } else {
        increment = -8;
        start_row_index = 6;
        end_row_index = 0;
    }

    let current_index = from_square / 8;
    if current_index != end_row_index {
        let all_pieces = game.get_occupied_bitboard();
        // One square forwards
        let mut target_square = from_square as isize + increment;
        let mut target_bit = onebit_index_to_bit(target_square as usize);
        if (target_bit & all_pieces) == 0 {
            pawn_moves.push(Move {
                from_square: from_square,
                to_square: target_square as usize,
                promotion: None,
                capture_square: None,
            });

            // Two squares forward
            if current_index == start_row_index {
                target_square += increment;
                target_bit = onebit_index_to_bit(target_square as usize);
                if (target_bit & all_pieces) == 0 {
                    pawn_moves.push(Move {
                        from_square: from_square,
                        to_square: target_square as usize,
                        promotion: None,
                        capture_square: None,
                    })
                }
            }
        }

        let enemy_pieces = game.get_enemy_piece_bitboard();
        // Left diagonal capture
        if from_square % 8 > 0 {
            let left_diagonal_target_square = from_square as isize + increment - 1;
            let left_diagonal_target_bit = onebit_index_to_bit(left_diagonal_target_square as usize);

            if (left_diagonal_target_bit & enemy_pieces) != 0 {
                pawn_moves.push(Move {
                    from_square: from_square,
                    to_square: left_diagonal_target_square as usize,
                    promotion: None,
                    capture_square: Some(left_diagonal_target_square as usize),
                });
            }
        }

        // Right diagonal capture
        if from_square % 8 < 7 {
            let right_diagonal_target_square = from_square as isize + increment + 1;
            let right_diagonal_target_bit = onebit_index_to_bit(right_diagonal_target_square as usize);

            if (right_diagonal_target_bit & enemy_pieces) != 0 {
                pawn_moves.push(Move {
                    from_square: from_square,
                    to_square: right_diagonal_target_square as usize,
                    promotion: None,
                    capture_square: Some(right_diagonal_target_square as usize),
                });
            }
        }

        // En passant
        match game.en_passant {
            Some(en_passant) => {
                let en_passant_onebit_index = bit_to_onebit_index(en_passant);
                let mut king_bit = 0u64;
                if let Some(king) = game.pieces.iter().find(|p| p.piece_type == PieceType::King && p.colour == game.active_colour) {
                    king_bit = king.bit;
                }

                // Left diagonal
                if from_square % 8 > 0 {
                    let left_diagonal_target_square = from_square as isize + increment - 1;
                    let pawns_bits = onebit_index_to_bit(from_square - 1) | onebit_index_to_bit(from_square);
                    let occupied_without_pawns = game.get_occupied_bitboard() & !pawns_bits;

                    if left_diagonal_target_square == en_passant_onebit_index as isize {
                        // make sure doesn't leave king in check
                        let horizontal_moves = get_horizontal_moves_through_en_passant_pawns(from_square, occupied_without_pawns, king_bit);
                        let horizontal_moves_without_pawns_or_king = horizontal_moves & occupied_without_pawns & !king_bit;
                        // if pawns are not on same row as king OR there isn't a rook or queen at the other end of the horizontal move from the king through the pawns
                        if horizontal_moves & pawns_bits == 0 || game.pieces.iter().all(|p|
                            p.bit & horizontal_moves_without_pawns_or_king == 0 || ![PieceType::Queen, PieceType::Rook].contains(&p.piece_type)
                        ) {
                            pawn_moves.push(Move {
                                from_square: from_square,
                                to_square: en_passant_onebit_index,
                                promotion: None,
                                capture_square: Some(from_square - 1),
                            });        
                        }
                        
                    }
                }

                // Right diagonal
                if from_square % 8 < 7 {
                    let right_diagonal_target_square = from_square as isize + increment + 1;
                    let pawns_bits = onebit_index_to_bit(from_square + 1) | onebit_index_to_bit(from_square);
                    let occupied_without_pawns = game.get_occupied_bitboard() & !pawns_bits;

                    if right_diagonal_target_square == en_passant_onebit_index as isize {
                        if right_diagonal_target_square == en_passant_onebit_index as isize {
                            // make sure doesn't leave king in check
                            let horizontal_moves = get_horizontal_moves_through_en_passant_pawns(from_square, occupied_without_pawns, king_bit);
                            let horizontal_moves_without_pawns_or_king = horizontal_moves & occupied_without_pawns & !king_bit;
                            // if pawns are not on same row as king OR there isn't a rook or queen at the other end of the horizontal move from the king through the pawns
                            if horizontal_moves & pawns_bits == 0 || game.pieces.iter().all(|p|
                                p.bit & horizontal_moves_without_pawns_or_king == 0 || ![PieceType::Queen, PieceType::Rook].contains(&p.piece_type)
                            ) {
                                pawn_moves.push(Move {
                                    from_square: from_square,
                                    to_square: en_passant_onebit_index,
                                    promotion: None,
                                    capture_square: Some(from_square + 1),
                                });        
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        for pawn_move in pawn_moves {
            if (game.active_colour == White && pawn_move.to_square >= 56) || (game.active_colour == Colour::Black && pawn_move.to_square < 8) {
                for promotion_piece in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight].iter() {
                    possible_moves.push(Move {
                        from_square: pawn_move.from_square,
                        to_square: pawn_move.to_square,
                        promotion: Some(*promotion_piece),
                        capture_square: pawn_move.capture_square,
                    })
                }
            } else {
                possible_moves.push(pawn_move);
            }
        }
    }

    possible_moves
}

fn get_horizontal_moves_through_en_passant_pawns(from_square: usize, occupied_without_pawns: u64, king_bit: u64) -> u64{
    let mut horizontal_moves = 0u64;
    let king_square = bit_to_onebit_index(king_bit);
    if king_square < from_square {
        // king is left of en passant pawns
        horizontal_moves = calculate_sliding_attacked_squares_including_own(
            ROOK_ATTACK_MASKS[1][king_square],
            occupied_without_pawns,
            1,
        );
    } else {
        // king is right of en passant pawns
        horizontal_moves = calculate_sliding_attacked_squares_including_own(
            ROOK_ATTACK_MASKS[3][king_square],
            occupied_without_pawns,
            3,
        );
    }
    horizontal_moves
}