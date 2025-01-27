use std::collections::VecDeque;
use std::io;
use std::io::Write;
use crate::game::{Colour, Game, Piece, PieceType, Square};
use crate::moves::{generate_moves, Move};

static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8,
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33
];

pub static _STARTING_FEN_STR: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub static _PROMOTION_FEN_STR: &str = "1n2k21/PPP5/4PPPP/8/8/8/8/RNBQKBNR w KQkq - 0 1";
pub static _AMBIGUOUS_FEN_STR: &str = "3r3r/2k5/8/R7/4Q2Q/8/8/RK5Q w KQkq - 0 1";
pub static _CASTLING_FEN_STR: &str = "5k2/8/8/8/8/8/2R5/R3K2R w KQkq - 0 1";
pub static _ENDGAME_FEN_STR: &str = "1k6/7P/8/8/8/8/8/RNBQKBNR w KQkq - 0 1";
pub static _LOSING_FEN_STR: &str = "1k6/q1qq4/8/8/8/6P1/8/1K5Q w KQkq - 0 1";
pub static _SIMPLE_FEN_STR: &str = "r3k3/8/8/8/8/8/8/R2QK3 w KQkq - 0 1";
pub static _DRAW_FEN_STR: &str = "4kn1n/8/8/8/8/8/8/4K3 w KQkq - 0 1";
pub static _PERFT_2_FEN_STR: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub static _PERFT_3_FEN_STR: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
pub static _PERFT_4_FEN_STR: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
pub static _PERFT_5_FEN_STR: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
pub static _PERFT_6_FEN_STR: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 1";
pub static _CHECK_OPTIMISATION_FEN_STR: &str = "r3k3/1p3p2/p2q2p1/bn3P2/1N2PQP1/1B6/3K1R1r/3R4 w q - 0 1";
pub static _LEGAL_TEST_FEN_STR: &str = "8/4k3/8/8/4R3/8/8/4K3 b - - 0 1";
pub static _LEGAL_TEST_2_FEN_STR: &str = "4k3/6N1/5b2/4R3/8/8/8/4K2R b - - 0 1";
pub static _LEGAL_TEST_3_FEN_STR: &str = "4k3/8/6n1/4R3/8/8/8/4K2R b - - 0 1";
pub static _LEGAL_TEST_4_FEN_STR: &str = "8/8/8/2k5/3Pp3/8/8/4K3 b - d3 0 1";
pub static _LEGAL_TEST_5_FEN_STR: &str = "8/8/8/1k6/3Pp3/8/8/4KQ2 b - d3 0 1";
pub static _LEGAL_TEST_6_FEN_STR: &str = "4k3/8/4r3/8/8/4Q3/8/2K5 b - - 0 1";
pub static _LEGAL_TEST_7_FEN_STR: &str = "8/8/8/8/k2Pp2Q/8/8/3K4 b - d3 0 1";
pub static _ENDGAME_1_FEN_STR: &str = "1r6/2r5/3k4/8/K7/8/8/8 w - - 0 1";
pub static _ENDGAME_2_FEN_STR: &str = "3r4/3r4/3k4/8/3K4/8/8/8 w - - 0 1";
pub static _ENDGAME_4_FEN_STR: &str = "8/3K4/4P3/8/8/8/6k1/7q w - - 0 1";

pub fn get_players_loop() -> i8 {
    loop {
        print!("How many players (0, 1 or 2): ");
        io::stdout().flush().unwrap();
        let mut players_input = String::new();
        io::stdin().read_line(&mut players_input).unwrap();
        players_input = players_input.trim().to_string();
        if players_input == "0" {
            return 0
        } else if players_input == "1" {
            return 1
        } else if players_input == "2" {
            return 2
        }
    }
}

#[allow(dead_code)]
pub fn bit_to_coords(bit: u64) -> Result<String, String> {
    if bit == 0 {
        Err("No piece present!".to_string())
    } else {
        let onebit_index = bit_to_onebit_index(bit);
        Ok(onebit_index_to_coords(onebit_index))
    }
}

pub fn coords_to_bit(coords: &str) -> Result<u64, String> {
    if let Ok(onebit_index) = coords_to_onebit_index(coords) {
        return Ok(onebit_index_to_bit(onebit_index))
    }
    return Err(format!("Invalid coords: {}", coords));
}

pub fn coords_to_onebit_index(coords: &str) -> Result<usize, String> {
    if coords.len() != 2 {
        return Err(format!("Invalid length: {}, string: '{}'", coords.len(), coords));
    }

    let bytes = coords.as_bytes();
    let byte0 = bytes[0];
    if byte0 < 97 || byte0 >= 97 + 8 {
        return Err(format!("Invalid column character: {}, string: '{}'", byte0 as char, coords));
    }
    let column = (byte0 - 97) as u32;

    let byte1 = bytes[1];
    let row;

    match (byte1 as char).to_digit(10) {
        Some(number) => if number < 1 || number > 8 {
            return Err(format!("Invalid row character: {}, string: '{}'", byte1, coords));
        } else {
            row = number - 1
        },
        None => return Err(format!("Invalid row character: {}, string: '{}'", byte1, coords)),
    }
    let onebit_index = row * 8 + column;
    Ok(onebit_index as usize)
}

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn onebit_index_to_coords(onebit_index: usize) -> String {
    let column = onebit_index % 8;
    let row = onebit_index / 8 + 1;
    format!("{}{}", COL_MAP[column], row)
}

pub fn onebit_index_to_bit(onebit_index: usize) -> u64 {
    1u64 << onebit_index
}

pub fn bit_to_onebit_index(bit: u64) -> usize {
    let remainder: usize = (bit % 67) as usize;
    MOD67TABLE[remainder]
}

pub fn split_on(s: &str, sep: char) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == sep {
            return (&s[0..i], &s[i + 1..]);
        }
    }
    (&s[..], "")
}


pub fn get_piece_index(square: &Square) -> Option<usize> {
    match square {
        Square::Occupied(piece_index) => Some(*piece_index),
        Square::Empty => None,
    }
}

#[allow(non_snake_case, unused_variables, unused_mut)]
pub fn parse_FEN_row(row: &str, mut piece_index: usize, mut onebit_index: usize) -> (Vec<Piece>, VecDeque<Square>) {
    let mut pieces = Vec::new();
    let mut squares = VecDeque::new();

    let mut colour;


    macro_rules! add_piece {
        ($piece_type:ident) => {
            {
                let piece = Piece {
                        colour: colour,
                        bit: (1 as u64) << onebit_index,
                        piece_type: PieceType::$piece_type,
                        taken: false,
                    };
                    let square = Square::Occupied(piece_index);
                    pieces.push(piece);
                    squares.push_front(square);
                    onebit_index += 1;
                    piece_index += 1;
            }
        };
    }
    for ch in row.chars() {
        let is_upper = ch.is_ascii_uppercase();
        colour = if is_upper { Colour::White } else { Colour::Black };
        match ch.to_ascii_lowercase() {
            'r' => {add_piece!(Rook)},
            'b' => {add_piece!(Bishop)},
            'n' => {add_piece!(Knight)},
            'q' => {add_piece!(Queen)},
            'k' => {add_piece!(King)},
            'p' => {add_piece!(Pawn)},
            num => {
                match num.to_digit(10) {
                    None => panic!("Invalid input: {}", num),
                    Some(number) => for _i in 0..number {
                        squares.push_front(Square::Empty);
                        onebit_index += 1;
                    }
                }
            }
        }
    }
    (pieces, squares)
}

pub fn parse_algebraic_move(move_input: &str, game: &mut Game) -> Option<Move> {
    let mut possible_matches = vec![];
    let possible_moves = generate_moves(game);
    for possible_move in possible_moves {
        if move_to_ambiguous_algebraic_notation(game, possible_move) == Some(move_input.to_owned().to_ascii_lowercase()) {
            possible_matches.push(possible_move);
        }
        if move_to_unambiguous_algebraic_notation(game, possible_move) == Some(move_input.to_owned().to_ascii_lowercase()) {
            possible_matches.push(possible_move);
        }
    }
    if possible_matches.len() == 1 {
        return Some(possible_matches[0])
    } else if possible_matches.len() > 1 {
        print_board(&game);
        // If all matches are just the same pawn promoting to different pieces:
        if possible_matches.iter().all(|m| m.from_square == possible_matches.iter().next().unwrap().from_square) {
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
            if let Some(matched) = possible_matches.iter().find(|&m| m.promotion == promotion_piece_type) {
                return Some(*matched);
            }
        } else {
            println!("Move is ambiguous, please double disambiguate (e.g. Qh4e1)");
            return None
        }
    }
    print_board(&game);
    println!("Invalid move, use algebraic notation without indication of captures (e.g. Nc3)");
    return None
}

pub fn move_to_unambiguous_algebraic_notation(game: &Game, possible_move: Move) -> Option<String> {
    let from_bit = onebit_index_to_bit(possible_move.from_square);
    if let Some(piece) = game.pieces.iter().find(|p| p.taken == false && p.bit == from_bit && p.colour == game.active_colour) {
        let mut algebraic_move = "".to_owned();
        match piece.piece_type {
            PieceType::Bishop => algebraic_move.push_str("b"),
            PieceType::Knight => algebraic_move.push_str("n"),
            PieceType::Rook => algebraic_move.push_str("r"),
            PieceType::King => algebraic_move.push_str("k"),
            PieceType::Queen => algebraic_move.push_str("q"),
            PieceType::Pawn => (),
        }
        let from_coords = onebit_index_to_coords(possible_move.from_square);
        let to_coords = onebit_index_to_coords(possible_move.to_square);
        algebraic_move.push_str(&from_coords);
        algebraic_move.push_str(&to_coords);
        Some(algebraic_move)

    } else {
        return None
    }
}

fn move_to_ambiguous_algebraic_notation(game: &Game, possible_move: Move) -> Option<String> {
    let from_bit = onebit_index_to_bit(possible_move.from_square);
    if let Some(piece) = game.pieces.iter().find(|p| p.taken == false && p.bit == from_bit && p.colour == game.active_colour) {
        let mut algebraic_move = "".to_owned();
        match piece.piece_type {
            PieceType::Bishop => algebraic_move.push_str("b"),
            PieceType::Knight => algebraic_move.push_str("n"),
            PieceType::Rook => algebraic_move.push_str("r"),
            PieceType::King => algebraic_move.push_str("k"),
            PieceType::Queen => algebraic_move.push_str("q"),
            PieceType::Pawn => (),
        }
        let to_coords = onebit_index_to_coords(possible_move.to_square);
        algebraic_move.push_str(&to_coords);
        Some(algebraic_move)

    } else {
        return None
    }
}


pub fn print_board(game: &Game) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", game.to_string());
}

pub fn bitboard_to_indices(bitboard: u64) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut bits = bitboard;
    while bits != 0 {
        let lsb = bits.trailing_zeros() as usize;
        indices.push(lsb);
        bits &= bits - 1;
    }
    indices
}

#[allow(dead_code)]
pub fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            let bit = (bitboard >> square) & 1;
            print!("{} ", bit);
        }
        println!();
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_on_space_works() {
        let test_string = "A B C D";
        let (should_be_a, _rest) = split_on(test_string, ' ');
        assert_eq!(should_be_a, "A");
        let (_should_be_b, _rest) = split_on(test_string, ' ');
    }

    #[test]
    fn split_on_ascii_works() {
        for i in 0..128 {
            let ch = char::from(i);
            if ch == 'A' {
                continue;
            }
            let test_string = format!("AA{}BB{}CC{}DD", ch, ch, ch);
            let (should_be_a, rest) = split_on(&test_string, ch);
            assert_eq!(should_be_a, "AA", "{}, {}, {}", test_string, ch, i);
            assert_eq!(rest, &format!("BB{}CC{}DD", ch, ch));
        }
    }
}