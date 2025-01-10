use bitflags::bitflags;
use std::collections::VecDeque;
use std::io;
use std::io::Write;
use crate::utils::*;
use crate::moves::*;

// e.g.s:
// coords: e4
// bit: 0000...0000000100000000000 (2^12)
// onebit_index = 12 (0 to 63)
// piece_index = count of piece (0 to 31)
// position: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 (board state)


// TODO: generate moves for each piece for a given board state
// and then use these to only allow legal moves.
// later use these for the engine to calculate good and bad moves
// use bitboard per piece type?
// knight
// king
// rook
// bishop
// queen
// pawn
// castling
// en passant
// check
// checkmate
// stalemate
// repetition draws

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Colour {
    White,
    Black
}

#[derive(Debug, PartialEq, Clone)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

#[derive(Debug, PartialEq, Clone)]
pub struct Piece {
    pub(crate) bit: u64,
    pub(crate) colour: Colour,
    pub(crate) piece_type: PieceType,
    pub(crate) taken: bool,
}

#[derive(Debug)]
pub enum Square {
    Empty,
    Occupied(usize),
}

pub struct Game {
    pub pieces: Vec<Piece>,
    pub squares: Vec<Square>,
    pub active_colour: Colour,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<u64>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
    possible_moves: Vec<Move>,
}

bitflags! {
    #[derive(Debug)]
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WHITEKINGSIDE = 1 << 0;
        const WHITEQUEENSIDE = 1 << 1;
        const BLACKKINGSIDE = 1 << 2;
        const BLACKQUEENSIDE = 1 << 3;
        const ALL = Self::WHITEKINGSIDE.bits() | Self::WHITEQUEENSIDE.bits() | Self::BLACKKINGSIDE.bits() | Self::BLACKQUEENSIDE.bits();
    }
}

impl Game {
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
            if let Some(_possible_move) = self.possible_moves.iter().find(|&m| m.to_square == i ) {
                background_colour = "\x1b[42m";
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

    #[allow(non_snake_case)]
    fn read_FEN(fen: &str) -> Game {
        let mut game = Game {
            pieces: vec![],
            squares: vec![],
            active_colour: Colour::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            possible_moves: vec![]
        };

        let (position, rest) = split_on(fen, ' ');

        let mut deque_squares = VecDeque::new();
        let mut piece_index = 0;
        let mut onebit_index = 64;

        for row in position.splitn(8, |ch| ch == '/') {
            onebit_index -= 8;
            let (pieces, squares) = parse_FEN_row(&row, piece_index, onebit_index);
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
            s => match coords_to_bit(s) {
                Err(msg) => panic!("{}", msg),
                Ok(bit) => game.en_passant = Some(bit),
            }
        }

        let (halfmove_clock, rest) = split_on(rest, ' ');
        match halfmove_clock.parse() {
            Ok(num) => game.halfmove_clock = num,
            Err(_) => panic!("Invalid halfmove: '{}'", halfmove_clock),
        }

        let (fullmove_number, _rest) = split_on(rest, ' ');
        match fullmove_number.parse() {
            Ok(num) => game.fullmove_number = num,
            Err(_) => panic!("Invalid fullmove: '{}'", fullmove_number),
        }
        game
    }
}

#[allow(non_snake_case)]
fn parse_FEN_row(row: &str, mut piece_index: usize, mut onebit_index: usize) -> (Vec<Piece>, VecDeque<Square>) {
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

impl Piece {
    fn to_string(&self) -> String {
        if self.colour == Colour::White {
            let result = match self.piece_type {
                PieceType::Pawn => "\x1b[97m ♟ ",
                PieceType::Bishop => "\x1b[97m ♝ ",
                PieceType::Knight => "\x1b[97m ♞ ",
                PieceType::Rook => "\x1b[97m ♜ ",
                PieceType::Queen => "\x1b[97m ♛ ",
                PieceType::King => "\x1b[97m ♚ ",
            }.to_string();
            result
        } else {
            let result = match self.piece_type {
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
    game.possible_moves = generate_moves(&mut game);

    print_board(&game);

    loop {
        println!("Move {:?} ({:?}):", game.fullmove_number, game.active_colour);
        print!("Piece coordinates: ");
        io::stdout().flush().unwrap();
        let mut start_input = String::new();
        io::stdin().read_line(&mut start_input).unwrap();
        start_input = start_input.trim().to_string();

        if let Ok(start_bit) = coords_to_bit(&start_input) {
            let start_onebit_index = bit_to_onebit_index(start_bit);
            if let Some(start_piece_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == start_bit && p.colour == game.active_colour) {
                print!("Target coordinates: ");
                io::stdout().flush().unwrap();
                let mut end_input = String::new();
                io::stdin().read_line(&mut end_input).unwrap();
                end_input = end_input.trim().to_string();

                if let Ok(end_bit) = coords_to_bit(&end_input) {
                    let end_onebit_index = bit_to_onebit_index(end_bit);
                    let input_move = Move {
                        from_square: start_onebit_index,
                        to_square: end_onebit_index,
                    };
                    if game.possible_moves.contains(&input_move) {
                        if let Some(target_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == end_bit) {
                            game.pieces[target_index].taken = true;
                        }
                        let piece_index = get_piece_index(&game.squares[start_onebit_index]);
                        game.squares[end_onebit_index] = Square::Occupied(piece_index.unwrap());
                        game.squares[start_onebit_index] = Square::Empty;
                        game.pieces[start_piece_index].bit = end_bit;
                        if game.active_colour == Colour::Black {
                            game.fullmove_number += 1;
                        }
                        game.active_colour = match game.active_colour {
                            Colour::White => Colour::Black,
                            Colour::Black => Colour::White,
                        };

                        game.possible_moves = generate_moves(&mut game);
                        print_board(&game);
                    } else {
                        println!("Invalid move");
                    }

                }
            } else {
                print_board(&game);
                println!("No {:?} piece at {}", game.active_colour, start_input);
            }
        }
    }
}

pub fn print_board(game: &Game) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", game.to_string());
}