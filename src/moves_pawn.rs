use crate::game::{Colour, Game, PieceType};
use crate::game::Colour::White;
use crate::moves::Move;
use crate::utils::{bit_to_onebit_index, onebit_index_to_bit};

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
                        pawn_moves.push(Move {
                            from_square: from_square,
                            to_square: en_passant_onebit_index,
                            promotion: None,
                        })
                    }
                }

                // Right diagonal
                if from_square % 8 < 7 {
                    let left_diagonal_target_square = from_square as isize + increment + 1;

                    if left_diagonal_target_square == en_passant_onebit_index as isize {
                        pawn_moves.push(Move {
                            from_square: from_square,
                            to_square: en_passant_onebit_index,
                            promotion: None,
                        })
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
                    })
                }
            } else {
                possible_moves.push(pawn_move);
            }
        }
    }

    possible_moves
}
