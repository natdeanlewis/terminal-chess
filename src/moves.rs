use crate::game::{CastlingRights, Game, PieceType, Square};
use crate::moves_bishop::{
    generate_bishop_attacked_squares_including_own, generate_bishop_moves,
    generate_bishop_pinned_piece, generate_bishop_pinned_ray,
};
use crate::moves_king::{
    add_castle_moves, generate_king_attacked_squares_including_own, generate_legal_king_moves,
};
use crate::moves_knight::{generate_knight_attacked_squares_including_own, generate_knight_moves};
use crate::moves_pawn::{generate_pawn_attacked_squares_including_own, generate_pawn_moves};
use crate::moves_queen::{
    generate_queen_attacked_squares_including_own, generate_queen_moves,
    generate_queen_pinned_piece, generate_queen_pinned_ray,
};
use crate::moves_rook::{
    generate_rook_attacked_squares_including_own, generate_rook_moves, generate_rook_pinned_piece,
    generate_rook_pinned_ray,
};
use crate::utils::*;
use crate::Colour;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub(crate) from_square: usize,
    pub(crate) to_square: usize,
    pub promotion: Option<PieceType>,
    pub capture_square: Option<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MoveToUnmake {
    pub(crate) from_square: usize,
    pub(crate) to_square: usize,
    pub promotion: Option<PieceType>,
    pub captured_piece_index: Option<usize>,
    pub captured_piece_square: Option<usize>,
    pub previous_castling_rights: CastlingRights,
    pub previous_en_passant: Option<u64>,
    pub previous_colour_in_check: Option<Colour>,
    pub previous_last_move: Option<Move>,
    pub previous_castled_rook_piece_index: Option<usize>,
    pub previous_castled_rook_piece_from_square: Option<usize>,
    pub previous_castled_rook_piece_to_square: Option<usize>,
    pub position_key: String,
}

pub fn generate_pseudolegal_moves_without_castling(game: &mut Game) -> Vec<Move> {
    let mut possible_moves: Vec<Move> = Vec::new();
    for piece in &game.pieces {
        if piece.colour == game.active_colour && piece.taken == false {
            let from_square = bit_to_onebit_index(piece.bit);
            match piece.piece_type {
                PieceType::Pawn => {
                    possible_moves.extend(generate_pawn_moves(from_square, game));
                }
                PieceType::Knight => {
                    possible_moves.extend(generate_knight_moves(from_square, game));
                }
                PieceType::Bishop => {
                    possible_moves.extend(generate_bishop_moves(from_square, game));
                }
                PieceType::Rook => {
                    possible_moves.extend(generate_rook_moves(from_square, game));
                }
                PieceType::Queen => {
                    possible_moves.extend(generate_queen_moves(from_square, game));
                }
                PieceType::King => {
                    possible_moves.extend(generate_legal_king_moves(from_square, game));
                }
            }
        }
    }

    possible_moves
}

pub fn squares_attacked_by_colour_bitboard(game: &Game, attacking_colour: Colour) -> u64 {
    let mut attacked_squares = 0u64;
    let mut king_bit = 0u64;
    if let Some(king) = game
        .pieces
        .iter()
        .find(|p| p.piece_type == PieceType::King && p.colour == game.active_colour)
    {
        king_bit = king.bit;
    }

    let occupied = game.get_occupied_bitboard();
    // Exclude king bit from occupied so king can't just move directly away from a checking sliding piece
    let occupied_excluding_king = occupied & !king_bit;

    for piece in &game.pieces {
        if piece.colour == attacking_colour && piece.taken == false {
            let from_square = bit_to_onebit_index(piece.bit);
            match piece.piece_type {
                PieceType::Pawn => {
                    attacked_squares |=
                        generate_pawn_attacked_squares_including_own(from_square, attacking_colour);
                }
                PieceType::Knight => {
                    attacked_squares |= generate_knight_attacked_squares_including_own(from_square);
                }
                PieceType::Bishop => {
                    attacked_squares |= generate_bishop_attacked_squares_including_own(
                        from_square,
                        occupied_excluding_king,
                    );
                }
                PieceType::Rook => {
                    attacked_squares |= generate_rook_attacked_squares_including_own(
                        from_square,
                        occupied_excluding_king,
                    );
                }
                PieceType::Queen => {
                    attacked_squares |= generate_queen_attacked_squares_including_own(
                        from_square,
                        occupied_excluding_king,
                    );
                }
                PieceType::King => {
                    attacked_squares |= generate_king_attacked_squares_including_own(from_square);
                }
            }
        }
    }

    attacked_squares
}

pub fn pieces_giving_check_bitboard(game: &Game, opponent_colour: Colour) -> u64 {
    let mut pieces_giving_check = 0u64;
    let mut king_bit = 0u64;
    if let Some(king) = game
        .pieces
        .iter()
        .find(|p| p.piece_type == PieceType::King && p.colour == game.active_colour)
    {
        king_bit = king.bit;
    }
    let occupied = game.get_occupied_bitboard();

    for piece in &game.pieces {
        if piece.colour == opponent_colour && piece.taken == false {
            let from_square = bit_to_onebit_index(piece.bit);
            match piece.piece_type {
                PieceType::Pawn => {
                    let pawn_attacks =
                        generate_pawn_attacked_squares_including_own(from_square, opponent_colour);
                    if pawn_attacks & king_bit != 0 {
                        pieces_giving_check |= piece.bit;
                    }
                }
                PieceType::Knight => {
                    let knight_attacks =
                        generate_knight_attacked_squares_including_own(from_square);
                    if knight_attacks & king_bit != 0 {
                        pieces_giving_check |= piece.bit;
                    }
                }
                PieceType::Bishop => {
                    let bishop_attacks =
                        generate_bishop_attacked_squares_including_own(from_square, occupied);
                    if bishop_attacks & king_bit != 0 {
                        pieces_giving_check |= piece.bit;
                    }
                }
                PieceType::Rook => {
                    let rook_attacks =
                        generate_rook_attacked_squares_including_own(from_square, occupied);
                    if rook_attacks & king_bit != 0 {
                        pieces_giving_check |= piece.bit;
                    }
                }
                PieceType::Queen => {
                    let queen_attacks =
                        generate_queen_attacked_squares_including_own(from_square, occupied);
                    if queen_attacks & king_bit != 0 {
                        pieces_giving_check |= piece.bit;
                    }
                }
                _ => (),
            }
        }
    }

    pieces_giving_check
}

pub fn pinned_ray_bitboard(game: &Game, opponent_colour: Colour, pinned_piece_bit: u64) -> u64 {
    let mut pinned_ray_bitboard = 0u64;
    let mut king_bit = 0u64;
    if let Some(king) = game
        .pieces
        .iter()
        .find(|p| p.piece_type == PieceType::King && p.colour == game.active_colour)
    {
        king_bit = king.bit;
    }

    for piece in &game.pieces {
        if piece.colour == opponent_colour && piece.taken == false {
            match piece.piece_type {
                PieceType::Bishop => {
                    pinned_ray_bitboard |=
                        generate_bishop_pinned_ray(pinned_piece_bit, game, king_bit);
                }
                PieceType::Rook => {
                    pinned_ray_bitboard |=
                        generate_rook_pinned_ray(pinned_piece_bit, game, king_bit);
                }
                PieceType::Queen => {
                    pinned_ray_bitboard |=
                        generate_queen_pinned_ray(pinned_piece_bit, game, king_bit);
                }
                _ => (),
            }
        }
    }

    pinned_ray_bitboard
}

pub fn pinned_pieces_bitboard(game: &Game, opponent_colour: Colour) -> u64 {
    let mut pinned_pieces_bitboard = 0u64;
    let mut king_bit = 0u64;
    if let Some(king) = game
        .pieces
        .iter()
        .find(|p| p.piece_type == PieceType::King && p.colour == game.active_colour)
    {
        king_bit = king.bit;
    }

    for piece in &game.pieces {
        if piece.colour == opponent_colour && piece.taken == false {
            let from_square = bit_to_onebit_index(piece.bit);
            match piece.piece_type {
                PieceType::Bishop => {
                    pinned_pieces_bitboard |=
                        generate_bishop_pinned_piece(from_square, game, king_bit);
                }
                PieceType::Rook => {
                    pinned_pieces_bitboard |=
                        generate_rook_pinned_piece(from_square, game, king_bit);
                }
                PieceType::Queen => {
                    pinned_pieces_bitboard |=
                        generate_queen_pinned_piece(from_square, game, king_bit);
                }
                _ => (),
            }
        }
    }

    pinned_pieces_bitboard
}

fn generate_pseudolegal_moves(game: &mut Game) -> Vec<Move> {
    let mut possible_moves = generate_pseudolegal_moves_without_castling(game);

    if let Some(king) = game
        .pieces
        .iter()
        .find(|&p| p.piece_type == PieceType::King && p.colour == game.active_colour)
    {
        let king_square = bit_to_onebit_index(king.bit);
        possible_moves = add_castle_moves(king_square, possible_moves, game);
    }

    possible_moves
}

pub fn generate_moves(game: &mut Game) -> Vec<Move> {
    let possible_moves = generate_pseudolegal_moves(game);

    // Only include moves that don't result in a check on the active colour
    let mut new_possible_moves = vec![];

    // king moves are legal already, make sure other pieces don't move king INTO check
    let king_square;
    let mut king_bit = 0;
    if let Some(king) = game
        .pieces
        .iter()
        .find(|p| p.piece_type == PieceType::King && p.colour == game.active_colour)
    {
        king_bit = king.bit;
    }
    king_square = bit_to_onebit_index(king_bit);
    let opponent_colour = match game.active_colour {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
    };

    let pieces_giving_check = pieces_giving_check_bitboard(game, opponent_colour);
    let num_pieces_giving_check = bitboard_to_indices(pieces_giving_check).len();
    let pinned_pieces_bitboard = pinned_pieces_bitboard(game, opponent_colour);
    // TODO push all legal king moves here first for efficiency?
    // consider fewer pieces for moves to pins? On a diagonal ray: bishops, queens, and pawns (captures only).
    // On a non-diagonal ray: rooks, queens, and pawns (pushes only).

    for possible_move in possible_moves {
        let mut capture_mask = u64::MAX;
        let mut push_mask = u64::MAX;

        if possible_move.from_square == king_square {
            // 1: king moves (handled by king move gen)
            new_possible_moves.push(possible_move);
        } else {
            let piece_bit = onebit_index_to_bit(possible_move.from_square);
            if piece_bit & pinned_pieces_bitboard != 0 {
                // make sure to_square is along pin line
                let pinned_ray_bitboard = pinned_ray_bitboard(game, opponent_colour, piece_bit);

                push_mask &= pinned_ray_bitboard;
                capture_mask &= pinned_ray_bitboard;
            }

            if pieces_giving_check != 0 {
                // in check
                if num_pieces_giving_check > 1 {
                    //double check
                    continue; // only king moves valid if double check
                } else {
                    // single check and piece to move is not king
                    capture_mask &= pieces_giving_check;
                    if let Some(checking_piece) = game
                        .pieces
                        .iter()
                        .find(|p| p.bit == pieces_giving_check && !p.taken)
                    {
                        match checking_piece.piece_type {
                            // If the piece giving check is a slider, we can evade check by blocking it
                            PieceType::Bishop => {
                                push_mask &= generate_bishop_pinned_piece(
                                    bit_to_onebit_index(checking_piece.bit),
                                    game,
                                    king_bit,
                                )
                            }
                            PieceType::Rook => {
                                push_mask &= generate_rook_pinned_piece(
                                    bit_to_onebit_index(checking_piece.bit),
                                    game,
                                    king_bit,
                                )
                            }
                            PieceType::Queen => {
                                push_mask &= generate_queen_pinned_piece(
                                    bit_to_onebit_index(checking_piece.bit),
                                    game,
                                    king_bit,
                                )
                            }
                            // if the piece is not a slider, we can only evade check by capturing
                            _ => push_mask = 0u64,
                        }
                    }
                }
            }

            if let Some(capture_square) = possible_move.capture_square {
                if onebit_index_to_bit(capture_square) & capture_mask != 0
                    || onebit_index_to_bit(possible_move.to_square) & push_mask != 0
                {
                    new_possible_moves.push(possible_move);
                }
            } else if onebit_index_to_bit(possible_move.to_square) & push_mask != 0 {
                new_possible_moves.push(possible_move);
            }
        }
    }
    new_possible_moves
}

pub fn generate_capture_moves(game: &mut Game) -> Vec<Move> {
    let possible_moves = generate_moves(game);
    let mut capture_moves = Vec::new();
    for possible_move in possible_moves {
        let to_bit = onebit_index_to_bit(possible_move.to_square);
        if to_bit & game.get_occupied_bitboard() != 0 {
            capture_moves.push(possible_move);
        }
    }

    capture_moves
}

pub fn make_move(game: &mut Game, move_to_make: Move) -> MoveToUnmake {
    let start_bit = onebit_index_to_bit(move_to_make.from_square);
    let end_bit = onebit_index_to_bit(move_to_make.to_square);

    if let Some(start_piece_index) = game
        .pieces
        .iter()
        .position(|p| p.taken == false && p.bit == start_bit && p.colour == game.active_colour)
    {
        let move_to_unmake =
            make_non_pawn_promotion_move(game, move_to_make, start_piece_index, end_bit);
        // Promote first so check will be calculated if promoted piece puts king in check
        if let Some(promotion_piece) = move_to_make.promotion {
            game.pieces[start_piece_index].piece_type = promotion_piece;
        };
        return move_to_unmake;
    }
    panic!("No piece found at this move's start index")
}

pub fn unmake_move(game: &mut Game, move_to_unmake: MoveToUnmake) {
    let start_bit = onebit_index_to_bit(move_to_unmake.from_square);
    let end_bit = onebit_index_to_bit(move_to_unmake.to_square);

    // Locate the moved piece
    if let Some(piece_index) = game
        .pieces
        .iter()
        .position(|p| !p.taken && p.bit == end_bit && p.colour != game.active_colour)
    {
        let piece = &mut game.pieces[piece_index];

        // Handle promotion revert
        if move_to_unmake.promotion.is_some() {
            piece.piece_type = PieceType::Pawn;
        }

        // Revert piece position
        piece.bit = start_bit;
        game.squares[move_to_unmake.from_square] = Square::Occupied(piece_index);
        game.squares[move_to_unmake.to_square] = Square::Empty;

        // Restore captured piece, if any
        if let (Some(captured_piece_index), Some(captured_piece_square)) = (
            move_to_unmake.captured_piece_index,
            move_to_unmake.captured_piece_square,
        ) {
            let captured_piece = &mut game.pieces[captured_piece_index];
            captured_piece.taken = false;
            game.squares[captured_piece_square] = Square::Occupied(captured_piece_index);
        }

        // Restore castled rook, if any
        if let (Some(rook_index), Some(rook_from_square), Some(rook_to_square)) = (
            move_to_unmake.previous_castled_rook_piece_index,
            move_to_unmake.previous_castled_rook_piece_from_square,
            move_to_unmake.previous_castled_rook_piece_to_square,
        ) {
            let rook = &mut game.pieces[rook_index];
            rook.bit = onebit_index_to_bit(rook_from_square);
            game.squares[rook_from_square] = Square::Occupied(rook_index);
            game.squares[rook_to_square] = Square::Empty;
        }

        // Restore game state
        game.castling_rights = move_to_unmake.previous_castling_rights;
        game.en_passant = move_to_unmake.previous_en_passant;
        game.colour_in_check = move_to_unmake.previous_colour_in_check;
        game.last_move = move_to_unmake.previous_last_move;

        // Reverse the single increment make_move applied for this position.
        if let Some(count) = game.position_counts.get_mut(&move_to_unmake.position_key) {
            *count -= 1;
            if *count == 0 {
                game.position_counts.remove(&move_to_unmake.position_key);
            }
        }

        // Adjust fullmove counter if Black's turn is reverted
        if game.active_colour == Colour::White {
            game.fullmove_number -= 1;
        }

        // Swap active colour
        game.active_colour = match game.active_colour {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        };
    }
}

fn make_non_pawn_promotion_move(
    game: &mut Game,
    move_to_make: Move,
    start_piece_index: usize,
    end_bit: u64,
) -> MoveToUnmake {
    let mut move_to_unmake = MoveToUnmake {
        from_square: move_to_make.from_square,
        to_square: move_to_make.to_square,
        promotion: move_to_make.promotion,
        captured_piece_index: None,
        captured_piece_square: None,
        previous_castling_rights: game.castling_rights,
        previous_en_passant: game.en_passant,
        previous_colour_in_check: game.colour_in_check,
        previous_last_move: game.last_move,
        previous_castled_rook_piece_index: None,
        previous_castled_rook_piece_from_square: None,
        previous_castled_rook_piece_to_square: None,
        position_key: String::new(),
    };

    let move_distance = (move_to_make.to_square as isize - move_to_make.from_square as isize).abs();

    // Handle Castling
    if game.pieces[start_piece_index].piece_type == PieceType::King {
        // Remove castling rights for both sides
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

        if move_distance == 2 {
            let (rook_from, rook_to) = if move_to_make.to_square > move_to_make.from_square {
                // King-side
                (move_to_make.from_square + 3, move_to_make.from_square + 1)
            } else {
                // Queen-side
                (move_to_make.from_square - 4, move_to_make.from_square - 1)
            };

            if let Some(rook_piece_index) = get_piece_index(&game.squares[rook_from]) {
                let rook = &mut game.pieces[rook_piece_index];
                rook.bit = onebit_index_to_bit(rook_to);
                game.squares[rook_to] = Square::Occupied(rook_piece_index);
                game.squares[rook_from] = Square::Empty;

                move_to_unmake.previous_castled_rook_piece_index = Some(rook_piece_index);
                move_to_unmake.previous_castled_rook_piece_from_square = Some(rook_from);
                move_to_unmake.previous_castled_rook_piece_to_square = Some(rook_to);
            }
        }
    }

    // Handle Rook moves that affect castling rights
    if game.pieces[start_piece_index].piece_type == PieceType::Rook {
        match move_to_make.from_square {
            0 => game.castling_rights.remove(CastlingRights::WHITEQUEENSIDE),
            7 => game.castling_rights.remove(CastlingRights::WHITEKINGSIDE),
            56 => game.castling_rights.remove(CastlingRights::BLACKQUEENSIDE),
            63 => game.castling_rights.remove(CastlingRights::BLACKKINGSIDE),
            _ => {}
        }
    }

    // Handle En Passant Capture
    if let Some(en_passant_bit) = game.en_passant {
        if end_bit == en_passant_bit && game.pieces[start_piece_index].piece_type == PieceType::Pawn
        {
            let captured_square = if game.active_colour == Colour::White {
                move_to_make.to_square - 8
            } else {
                move_to_make.to_square + 8
            };

            if let Some(captured_piece_index) = get_piece_index(&game.squares[captured_square]) {
                game.pieces[captured_piece_index].taken = true;
                game.squares[captured_square] = Square::Empty;
                move_to_unmake.captured_piece_index = Some(captured_piece_index);
                move_to_unmake.captured_piece_square = Some(captured_square);
            }
        }
    }

    // Handle Standard Capture
    if let Some(target_index) = game
        .pieces
        .iter()
        .position(|p| !p.taken && p.bit == end_bit)
    {
        game.pieces[target_index].taken = true;
        game.squares[move_to_make.to_square] = Square::Occupied(target_index);
        move_to_unmake.captured_piece_index = Some(target_index);
        move_to_unmake.captured_piece_square =
            Some(bit_to_onebit_index(game.pieces[target_index].bit));

        // Remove captured rook's castling rights
        if game.pieces[target_index].piece_type == PieceType::Rook {
            match move_to_make.to_square {
                0 => game.castling_rights.remove(CastlingRights::WHITEQUEENSIDE),
                7 => game.castling_rights.remove(CastlingRights::WHITEKINGSIDE),
                56 => game.castling_rights.remove(CastlingRights::BLACKQUEENSIDE),
                63 => game.castling_rights.remove(CastlingRights::BLACKKINGSIDE),
                _ => {}
            }
        }
    }

    // Update the board
    game.squares[move_to_make.to_square] = Square::Occupied(start_piece_index);
    game.squares[move_to_make.from_square] = Square::Empty;
    game.pieces[start_piece_index].bit = end_bit;

    // Set En Passant if Pawn moved two squares
    game.en_passant =
        if game.pieces[start_piece_index].piece_type == PieceType::Pawn && move_distance == 16 {
            Some(onebit_index_to_bit(
                (move_to_make.from_square + move_to_make.to_square) / 2,
            ))
        } else {
            None
        };

    // Check if the opponent's king is in check
    let opponent_colour = match game.active_colour {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
    };

    if let Some(king) = game
        .pieces
        .iter()
        .find(|p| p.piece_type == PieceType::King && p.colour == opponent_colour)
    {
        let squares_attacked_by_active_colour =
            squares_attacked_by_colour_bitboard(game, game.active_colour);

        game.colour_in_check = if squares_attacked_by_active_colour & king.bit != 0 {
            Some(opponent_colour)
        } else {
            None
        };
    }

    // Increment fullmove number and switch active colour
    if game.active_colour == Colour::Black {
        game.fullmove_number += 1;
    }
    game.active_colour = opponent_colour;

    // Update position counts
    let position_key = Game::write_FEN_without_move_counts(game);
    *game
        .position_counts
        .entry(position_key.clone())
        .or_insert(0) += 1;
    move_to_unmake.position_key = position_key;

    move_to_unmake
}

pub fn calculate_sliding_attacked_squares_excluding_own(
    attack_mask: u64,
    occupied: u64,
    direction: usize,
    own_pieces: u64,
) -> u64 {
    let blockers = attack_mask & occupied;
    let mut truncated_mask = attack_mask;

    if blockers != 0 {
        match direction {
            0 | 1 => {
                // North/East (orthogonal), North-West/North-East (diagonal)t
                let first_blocker = blockers.trailing_zeros() as usize;
                let blocker_bit = 1u64 << first_blocker;
                if blocker_bit & own_pieces == 0 {
                    // Enemy piece, include
                    truncated_mask &= blocker_bit | (blocker_bit - 1);
                } else {
                    // Friendly piece, exclude
                    truncated_mask &= blocker_bit - 1;
                }
            }
            2 | 3 => {
                // South/West (orthogonal), South-East/South-West (diagonal)t
                let first_blocker = 63 - blockers.leading_zeros() as usize;
                let blocker_bit = 1u64 << first_blocker;
                if blocker_bit & own_pieces == 0 {
                    // Enemy piece, include
                    truncated_mask &= !(blocker_bit - 1);
                } else {
                    // Friendly piece, exclude
                    truncated_mask &= !blocker_bit & !(blocker_bit - 1);
                }
            }
            _ => panic!("Invalid direction"),
        };
    }
    truncated_mask
}

pub fn calculate_sliding_attacked_squares_including_own(
    attack_mask: u64,
    occupied: u64,
    direction: usize,
) -> u64 {
    let blockers = attack_mask & occupied;
    let mut truncated_mask = attack_mask;

    if blockers != 0 {
        match direction {
            0 | 1 => {
                // North/East (orthogonal), North-West/North-East (diagonal)t
                let first_blocker = blockers.trailing_zeros() as usize;
                let blocker_bit = 1u64 << first_blocker;
                truncated_mask &= blocker_bit | (blocker_bit - 1);
            }
            2 | 3 => {
                // South/West (orthogonal), South-East/South-West (diagonal)t
                let first_blocker = 63 - blockers.leading_zeros() as usize;
                let blocker_bit = 1u64 << first_blocker;
                truncated_mask &= !(blocker_bit - 1);
            }
            _ => panic!("Invalid direction"),
        };
    }
    truncated_mask
}

#[test]
fn perft_1() {
    let test_number = 1;
    let _perft_1_fen_str = _STARTING_FEN_STR;
    let expected_node_counts = [1, 20, 400, 8_902, 197_281, 4_865_609];

    let mut game = Game::initialize(_perft_1_fen_str);
    run_perft_test(&mut game, &expected_node_counts, test_number);
}

#[test]
fn perft_2() {
    let test_number = 2;
    let expected_node_counts = [1, 48, 2_039, 97_862, 4_085_603];

    let mut game = Game::initialize(_PERFT_2_FEN_STR);
    run_perft_test(&mut game, &expected_node_counts, test_number);
}

#[test]
fn perft_3() {
    let test_number = 3;
    let expected_node_counts = [1, 14, 191, 2_812, 43_238, 674_624];

    let mut game = Game::initialize(_PERFT_3_FEN_STR);
    run_perft_test(&mut game, &expected_node_counts, test_number);
}

#[test]
fn perft_4() {
    let test_number = 4;
    let expected_node_counts = [1, 6, 264, 9_467, 422_333];

    let mut game = Game::initialize(_PERFT_4_FEN_STR);
    run_perft_test(&mut game, &expected_node_counts, test_number);
}

#[test]
fn perft_5() {
    let test_number = 5;
    let expected_node_counts = [1, 44, 1_486, 62_379, 2_103_487];

    let mut game = Game::initialize(_PERFT_5_FEN_STR);
    run_perft_test(&mut game, &expected_node_counts, test_number);
}

#[test]
fn perft_6() {
    let test_number = 6;
    let expected_node_counts = [1, 46, 2_079, 89_890, 3_894_594];

    let mut game = Game::initialize(_PERFT_6_FEN_STR);
    run_perft_test(&mut game, &expected_node_counts, test_number);
}

fn perft_func(depth: u32, game: &mut Game) -> u32 {
    if depth == 0 {
        return 1;
    }
    let mut total = 0;

    let n_moves = generate_moves(game);
    for n_move in n_moves.iter() {
        let move_to_unmake = make_move(game, *n_move);
        let nodes: u32 = perft_func(depth - 1, game);
        total += nodes;
        unmake_move(game, move_to_unmake);
    }
    total
}

#[allow(unused)]
fn run_perft_test(game: &mut Game, expected_node_counts: &[u32], test_number: i32) {
    let mut running_total = 0;
    let now = Instant::now();

    for (depth, &expected_nodes) in expected_node_counts.iter().enumerate() {
        let nodes = perft_func(depth as u32, game);
        running_total += nodes;
        assert_eq!(nodes, expected_nodes, "Mismatch at depth {}", depth);
        // println!("Test {}: Depth {}: Success! Expected {} nodes, got {}", test_number, depth, expected_nodes, nodes);
    }

    let elapsed = now.elapsed();
    let nps = running_total as f64 / elapsed.as_secs_f64();
    println!("Overall Nodes Per Second: {}", nps);
}
