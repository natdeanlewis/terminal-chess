use bitflags::bitflags;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::io;
use std::io::Write;
use crate::evaluation::iterative_deepening_minimax;
use crate::utils::*;
use crate::moves::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Colour {
    White,
    Black
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub(crate) bit: u64,
    pub(crate) colour: Colour,
    pub(crate) piece_type: PieceType,
    pub(crate) taken: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Square {
    Empty,
    Occupied(usize),
}

#[derive(Clone)]
pub struct Game {
    pub pieces: Vec<Piece>,
    pub squares: Vec<Square>,
    pub active_colour: Colour,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<u64>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
    pub(crate) colour_in_check: Option<Colour>,
    pub(crate) last_move: Option<Move>,
    position_counts: HashMap<String, i32>,
    pub players: i8
}

bitflags! {
    #[derive(Debug, Clone, PartialEq, Copy)]
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WHITEKINGSIDE = 1 << 0;
        const WHITEQUEENSIDE = 1 << 1;
        const BLACKKINGSIDE = 1 << 2;
        const BLACKQUEENSIDE = 1 << 3;
        const ALL = Self::WHITEKINGSIDE.bits() | Self::WHITEQUEENSIDE.bits() | Self::BLACKKINGSIDE.bits() | Self::BLACKQUEENSIDE.bits();
    }
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


impl Game {
    pub fn initialize(fen_str: &str) -> Game {
        Game::read_FEN(fen_str)
    }

    pub fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();
        board.insert_str(0, "   a  b  c  d  e  f  g  h");
        for (i, square) in self.squares.iter().enumerate() {
            if i % 8 == 0 {
                temp.push_str(&format!("{} ", (i / 8) + 1));
            }

            let mut background_colour = if i % 2 == (i / 8) % 2 { "\x1b[48;5;130m" } else { "\x1b[48;5;172m" };

            // Last move highlighting
            if let Some(last_move) = self.last_move {
                if i == last_move.to_square {
                    background_colour = "\x1b[48;5;112m";
                }
                if i == last_move.from_square {
                    background_colour = "\x1b[48;5;149m";
                }
            }
            // Possible move highlighting:
            // for m in generate_moves(&mut self.clone()) {
            //     if i == m.to_square {
            //         background_colour = "\x1b[48;5;80m";
            //     }
            // }


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
                temp.push_str(&format!(" {}", (i / 8) + 1));
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
        }
        board.insert_str(0, "   a  b  c  d  e  f  g  h\n");

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
            colour_in_check: None,
            last_move: None,
            players: 1,
            position_counts: HashMap::new(),
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
        // TODO: set colour_in_check if in check
        game
    }

    #[allow(non_snake_case)]
    fn write_FEN_without_move_counts(game: &Game) -> String {
        let mut fen_string = "".to_owned();
        let mut consecutive_empty_squares: u8 = 0;
        let mut row_temp = "".to_owned();
        for (i, square) in game.squares.iter().enumerate() {
            match square {
                Square::Empty => {
                    consecutive_empty_squares += 1
                },
                Square::Occupied(piece_index) => {
                    if consecutive_empty_squares > 0 {
                        row_temp.push_str(&consecutive_empty_squares.to_string());
                    }
                    let piece = &game.pieces[*piece_index];
                    let lowercase_piece_char = match piece.piece_type {
                        PieceType::Pawn => 'p',
                        PieceType::Knight =>  'n',
                        PieceType::Bishop => 'b',
                        PieceType::Rook => 'r',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                    };
                    if piece.colour == Colour::White {
                        row_temp.push(lowercase_piece_char.to_ascii_uppercase());
                    } else {
                        row_temp.push(lowercase_piece_char);
                    }
                    consecutive_empty_squares = 0
                }
            }
            if (i + 1) % 8 == 0 {
                if consecutive_empty_squares > 0 {
                    row_temp.push_str(&consecutive_empty_squares.to_string());
                    consecutive_empty_squares = 0
                }
                // Second or later row
                if i >= 15 {
                    row_temp.push('/');
                }

                fen_string = row_temp.clone() + &fen_string;
                row_temp.clear();
            }
        }

        let colour_to_move = match game.active_colour {
            Colour::White => 'w',
            Colour::Black => 'b',
        };

        fen_string.push(' ');
        fen_string.push(colour_to_move);
        fen_string.push(' ');

        if game.castling_rights == CastlingRights::NONE {
            fen_string.push('-')
        } else {
            if game.castling_rights.contains(CastlingRights::WHITEKINGSIDE) {
                fen_string.push('K')
            }
            if game.castling_rights.contains(CastlingRights::WHITEQUEENSIDE) {
                fen_string.push('Q')
            }
            if game.castling_rights.contains(CastlingRights::BLACKKINGSIDE) {
                fen_string.push('k')
            }
            if game.castling_rights.contains(CastlingRights::BLACKQUEENSIDE) {
                fen_string.push('q')
            }
        }

        fen_string.push(' ');


        if let Some(en_passant) = game.en_passant {
            for char in bit_to_coords(en_passant).unwrap().chars() {
                fen_string.push(char);
            }
        } else {
            fen_string.push('-');
        }

        fen_string
    }

    pub fn get_friendly_piece_bitboard(&self) -> u64 {
        self.pieces
            .iter()
            .filter(|piece| piece.colour == self.active_colour && piece.taken == false)
            .fold(0u64, |bitboard, piece| bitboard | piece.bit)
    }

    pub fn get_enemy_piece_bitboard(&self) -> u64 {
        self.pieces
            .iter()
            .filter(|piece| piece.colour != self.active_colour && piece.taken == false)
            .fold(0u64, |bitboard, piece| bitboard | piece.bit)
    }

    pub fn get_occupied_bitboard(&self) -> u64 {
        self.pieces
            .iter()
            .filter(|piece| piece.taken == false)
            .fold(0u64, |bitboard, piece| bitboard | piece.bit)
    }
}

pub fn game_loop(mut game: Game) {
    let max_depth = 5;
    let fen_string = Game::write_FEN_without_move_counts(&game);
    *game.position_counts.entry(fen_string.clone()).or_insert(0) += 1;

    let mut possible_moves = generate_moves(&mut game);
    print_board(&game);

    loop {
        // if both sides have only a king with knights or bishops
        if game.pieces.iter().filter(|piece| !piece.taken && [PieceType::Queen, PieceType::Rook, PieceType::Pawn].contains(&piece.piece_type)).count() == 0 {
            if game.pieces.iter().filter(|piece| piece.colour == Colour::White && !piece.taken).count() <= 2 && game.pieces.iter().filter(|piece| piece.colour == Colour::Black && !piece.taken).count() <= 2 {
                println!{"Draw."};
                break
            }
        }

        let inactive_colour = match game.active_colour {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        };

        if possible_moves.len() == 0 {
            if game.colour_in_check == Some(game.active_colour) {
                println!{"Checkmate. {:?} wins.", inactive_colour};
            } else {
                println!{"Stalemate."};
            }
            break
        }
        if game.colour_in_check == Some(game.active_colour) {
            println!("Check.");
        }

        println!("Move {:?} ({:?}):", game.fullmove_number, game.active_colour);

        if game.players == 0 || (game.players == 1 && game.active_colour == Colour::Black) {
            println!("Thinking...");
            if let Some(best_move) = iterative_deepening_minimax(&mut game, max_depth) {
                make_move(&mut game, best_move);
                game.last_move = Some(best_move);
                possible_moves = generate_moves(&mut game);
                print_board(&game);

                let fen_string = Game::write_FEN_without_move_counts(&game);
                *game.position_counts.entry(fen_string.clone()).or_insert(0) += 1;
                if let Some(position_count) = game.position_counts.get(&fen_string) {
                    if *position_count == 5 {
                        println!{"Draw by fivefold repetition."};
                        break

                    }
                }
            }
        } else {
            print!("Enter move: ");
            io::stdout().flush().unwrap();
            let mut move_input = String::new();
            io::stdin().read_line(&mut move_input).unwrap();
            move_input = move_input.trim().to_string();

            if let Some(input_move) = parse_algebraic_move(&move_input, &mut game) {
                let start_bit = onebit_index_to_bit(input_move.from_square);
                if let Some(_start_piece_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == start_bit && p.colour == game.active_colour) {
                    make_move(&mut game, input_move);
                    game.last_move = Some(input_move);
                    possible_moves = generate_moves(&mut game);
                    print_board(&game);

                    let fen_string = Game::write_FEN_without_move_counts(&game);
                    *game.position_counts.entry(fen_string.clone()).or_insert(0) += 1;
                    if let Some(position_count) = game.position_counts.get(&fen_string) {
                        if *position_count == 5 {
                            println!{"Draw by fivefold repetition."};
                            break
                        }
                    }
                } else {
                    print_board(&game);
                    println!("Invalid move");
                }
            }
        }

    }
}