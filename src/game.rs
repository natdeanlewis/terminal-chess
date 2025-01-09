use bitflags::bitflags;
use std::collections::VecDeque;
use std::io;
use std::io::Write;
use crate::utils::*;
type PiecePosition = u64;
// e.g.s:
// position: e4
// bit: 0000...0000000100000000000 (2^12)
// onebit_index = 12 (0 to 63)
// piece_index = count of piece (1 to 32)

pub fn bit_to_position(bit: PiecePosition) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present!".to_string());
    } else {
        let onebit_index = bit_scan(bit);
        return Ok(onebit_index_to_position(onebit_index));
    }
}

pub fn position_to_bit(position: &str) -> Result<PiecePosition, String> {
    if position.len() != 2 {
        return Err(format!("Invalid length: {}, string: '{}'", position.len(), position));
    }

    let bytes = position.as_bytes();
    let byte0 = bytes[0];
    if byte0 < 97 || byte0 >= 97 + 8 {
        return Err(format!("Invalid column character: {}, string: '{}'", byte0 as char, position));
    }
    let column = (byte0 - 97) as u32;

    let byte1 = bytes[1];
    let row;

    match (byte1 as char).to_digit(10) {
        Some(number) => if number < 1 || number > 8 {
            return Err(format!("Invalid row character: {}, string: '{}'", byte1, position));
        } else {
            row = number - 1
        },
        None => return Err(format!("Invalid row character: {}, string: '{}'", byte1, position)),
    }
    let square_number = row * 8 + column;
    let bit = (1 as u64) << square_number;
    Ok(bit)
}

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn onebit_index_to_position(onebit_index: usize) -> String {
    let column = onebit_index % 8;
    let row = onebit_index / 8 + 1;
    format!("{}{}", COL_MAP[column], row)
}

pub fn position_to_onebit_index(coords: &str) -> Option<(u8)> {
    let mut chars = coords.chars();
    let column_char = chars.next()?;
    let row_char = chars.next()?;

    let col_index = COL_MAP.iter().position(|&c| c == column_char)? as u8;
    let row = row_char.to_digit(10)? as u8;
    let row_index = row - 1;
    Some(8 * row_index + col_index)
}
pub fn coords_to_position(coords: &str) -> Option<(u64)>  {
    let bit = position_to_onebit_index(coords)?;
    let position = 1u64 << bit;
    Some(position)
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Colour {
    White,
    Black
}

#[derive(Debug, PartialEq, Clone)]
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

#[derive(Debug, PartialEq, Clone)]
struct Piece {
    position: PiecePosition,
    colour: Colour,
    piece_type: PieceType,
}

#[derive(Debug)]
enum Square {
    Empty,
    Occupied(usize),
}

pub struct Game {
    pub pieces: Vec<Piece>,
    pub squares: Vec<Square>,
    pub active_colour: Colour,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<PiecePosition>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
    pub selected_square: Option<u8>,
}

bitflags! {
    #[derive(Debug)]
    struct CastlingRights: u8 {
        const NONE = 0;
        const WHITEKINGSIDE = 1 << 0;
        const WHITEQUEENSIDE = 1 << 1;
        const BLACKKINGSIDE = 1 << 2;
        const BLACKQUEENSIDE = 1 << 3;
        const ALL = Self::WHITEKINGSIDE.bits() | Self::WHITEQUEENSIDE.bits() | Self::BLACKKINGSIDE.bits() | Self::BLACKQUEENSIDE.bits();
    }
}

impl Game {
    fn push_piece_and_square(&mut self, position: usize, colour: Colour, piece_type: PieceType, piece_index: &mut usize) {
        self.pieces.push(Piece {
            position: 1u64 << position,
            colour: colour,
            piece_type: piece_type,
        });
        self.squares.push(Square::Occupied(*piece_index));
        *piece_index += 1;
    }

    fn push_empty_square(&mut self) {
        self.squares.push(Square::Empty);
    }
    pub fn initialize() -> Game {
        let starting_fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Game::read_FEN(starting_fen_str)
    }

    fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();
        board.insert_str(0, "   a  b  c  d  e  f  g  h");
        for (i, square) in self.squares.iter().enumerate() {
            if i % 8 == 0 {
                temp.push_str(&format!("{} ", (i / 8) + 1));
            }

            let mut background_colour = if i % 2 == (i / 8) % 2 { "\x1b[48;5;130m" } else { "\x1b[48;5;172m" };
            if Some(i as u8) == self.selected_square {
                background_colour = "\x1b[48;5;70m";
            }
            temp.push_str(background_colour);
            match square {
                Square::Empty => {

                    temp.push_str("   ")
                },
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
            }
            let colour_end = "\x1b[0m";
            temp.push_str(colour_end);

            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
        }
        board.insert_str(0, &temp);
        board
    }

    fn read_FEN(fen: &str) -> Game {
        let mut game = Game {
            pieces: vec![],
            squares: vec![],
            active_colour: Colour::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            selected_square: None,
        };

        let (position, rest) = split_on(fen, ' ');

        let mut deque_squares = VecDeque::new();
        let mut piece_index = 0;
        let mut piece_position = 64;

        for row in position.splitn(8, |ch| ch == '/') {
            piece_position -= 8;
            let (pieces, squares) = parse_row(&row, piece_index, piece_position);
            for p in pieces {
                game.pieces.push(p);
                piece_index += 1;
            }
            for s in squares {
                deque_squares.push_front(s);
            }
        }

        game.squares = Vec::from(deque_squares);

        let (colour_to_move, rest) = split_on(rest, ' ');
        game.active_colour = match colour_to_move {
            "w" => Colour::White,
            "b" => Colour::Black,
            _ => panic!("Unknown colour designator: '{}'", colour_to_move),
        };

        let (castling_rights, rest) = split_on(rest, ' ');
        let mut castling = CastlingRights::NONE;
        for ch in castling_rights.chars() {
            match ch {
                'K' => castling |= CastlingRights::WHITEKINGSIDE,
                'Q' => castling |= CastlingRights::WHITEQUEENSIDE,
                'k' => castling |= CastlingRights::BLACKKINGSIDE,
                'q' => castling |= CastlingRights::BLACKQUEENSIDE,
                '-' => (),
                _ => panic!("Unknown castling designator: '{}'", ch),
            }
        }
        game.castling_rights = castling;

        let (en_passant, rest) = split_on(rest, ' ');
        match en_passant {
            "-" => game.en_passant = None,
            s => match position_to_bit(s) {
                Err(msg) => panic!("{}", msg),
                Ok(bit) => game.en_passant = Some(bit),
            }
        }

        let (halfmove_clock, rest) = split_on(rest, ' ');
        match halfmove_clock.parse() {
            Ok(num) => game.halfmove_clock = num,
            Err(_) => panic!("Invalid halfmove: '{}'", halfmove_clock),
        }

        let (fullmove_number, rest) = split_on(rest, ' ');
        match fullmove_number.parse() {
            Ok(num) => game.fullmove_number = num,
            Err(_) => panic!("Invalid fullmove: '{}'", fullmove_number),
        }
        game
    }
}

fn parse_row(row: &str, mut piece_index: usize, mut piece_position: usize) -> (Vec<Piece>, VecDeque<Square>) {
    let mut pieces = Vec::new();
    let mut squares = VecDeque::new();

    let mut colour;


    macro_rules! add_piece {
        ($piece_type:ident) => {
            {
                let piece = Piece {
                        colour: colour,
                        position: (1 as u64) << piece_position,
                        piece_type: PieceType::$piece_type,
                    };
                    let square = Square::Occupied(piece_index);
                    pieces.push(piece);
                    squares.push_front(square);
                    piece_position += 1;
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
                    Some(number) => for i in 0..number {
                        squares.push_front(Square::Empty);
                        piece_position += 1;
                    }
                }
            }
        }
    }
    (pieces, squares)
}

impl Piece {
    fn to_string(&self) -> String {
        if self.colour == Colour::White {
            let mut result = match self.piece_type {
                PieceType::Pawn => "\x1b[97m ♟ ",
                PieceType::Bishop => "\x1b[97m ♝ ",
                PieceType::Knight => "\x1b[97m ♞ ",
                PieceType::Rook => "\x1b[97m ♜ ",
                PieceType::Queen => "\x1b[97m ♛ ",
                PieceType::King => "\x1b[97m ♚ ",
            }.to_string();
            result
        } else {
            let mut result = match self.piece_type {
                PieceType::Pawn => "\x1b[30m ♟ ",
                PieceType::Bishop => "\x1b[30m ♝ ",
                PieceType::Knight => "\x1b[30m ♞ ",
                PieceType::Rook => "\x1b[30m ♜ ",
                PieceType::Queen => "\x1b[30m ♛ ",
                PieceType::King => "\x1b[30m ♚ ",
            }.to_string();
            result
        }
    }
}

fn get_piece_index(square: &Square) -> Option<usize> {
    match square {
        Square::Occupied(piece_index) => Some(*piece_index),
        Square::Empty => None,
    }
}

pub fn game_loop(mut game: Game) {
    loop {
        println!("Move {:?} ({:?}):", game.fullmove_number, game.active_colour);
        print!("Piece coordinates: ");
        io::stdout().flush().unwrap();
        let mut start_input = String::new();
        io::stdin().read_line(&mut start_input).unwrap();
        start_input = start_input.trim().to_string();

        if let Ok(start_position) = position_to_bit(&start_input) {

            if let Some(start_square) = position_to_onebit_index(&start_input) {
                println!("{}, {}", start_input, start_square);
                if let Some(start_piece_index) = game.pieces.iter().position(|p| p.position == start_position && p.colour == game.active_colour) {
                    game.selected_square = Some(start_square);
                    print_board(&game);
                    print!(
                        "Target coordinates: ",
                    );
                    io::stdout().flush().unwrap();
                    let mut end_input = String::new();
                    io::stdin().read_line(&mut end_input).unwrap();
                    end_input = end_input.trim().to_string();

                    if let Ok(end_position) = position_to_bit(&end_input) {
                        if let Some(end_square) = position_to_onebit_index(&end_input) {
                            if let Some(target_index) = game.pieces.iter_mut().position(|p| p.position == end_position && p.colour != game.active_colour) {
                                game.squares[end_square as usize] = Square::Empty;
                                game.pieces[target_index].position = 0;
                            }
                            game.selected_square = None;

                            game.pieces[start_piece_index].position = end_position;
                            let piece_index = get_piece_index(&game.squares[start_square as usize]);
                            game.squares[start_square as usize] = Square::Empty;
                            game.squares[end_square as usize] = Square::Occupied(piece_index.unwrap());
                            if game.active_colour == Colour::Black {
                                game.fullmove_number += 1;
                            }
                            game.active_colour = match game.active_colour {
                                Colour::White => Colour::Black,
                                Colour::Black => Colour::White,
                            };
                            print_board(&game);
                        }
                    }
                } else {
                    print_board(&game);
                    println!("No {:?} piece at {}", game.active_colour, start_input);
                }
            }
        }
    }
}

pub fn print_board(game: &Game) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", game.to_string());
}
