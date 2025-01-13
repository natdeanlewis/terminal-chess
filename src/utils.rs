use std::collections::VecDeque;
use crate::game::{Colour, Game, Piece, PieceType, Square};
use crate::moves::Move;

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

pub fn parse_algebraic_move(move_input: &str, game: &Game) -> Option<Move> {
    let mut possible_matches = vec![];
    for possible_move in game.possible_moves.clone() {
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
        println!("Move is ambiguous, please double disambiguate (e.g. Qh4e1)");
        return None
    }
    print_board(&game);
    println!("Invalid move, use algebraic notation without indication of captures (e.g. Nc3)");
    return None
}

fn move_to_unambiguous_algebraic_notation(game: &Game, possible_move: Move) -> Option<String> {
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