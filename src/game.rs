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
// knight done
// king done
// rook done
// bishop done
// queen done
// pawn done
// castling done
// en passant done
// pawn promotions done
// algebraic move notation? done
// alert when check?
// check done
// checkmate done
// stalemate done
// dont' alow castling when ONLY king square is threatened (castling out of check)
// tests
// perft
// repetition draws
// optimisation
// board evaluation
// positional skewing
// search
// minimax
// alphabeta pruning
// play multiple colours
// mouse gui
// full algebraic move notation?

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
    possible_moves: Vec<Move>,
    selected_piece_square: Option<usize>,
    colour_in_check: Option<Colour>,
}

bitflags! {
    #[derive(Debug, Clone)]
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
        // let ambiguous_fen_str = "3r3r/2k5/8/R7/4Q2Q/8/8/RK5Q w KQkq - 0 1";
        let starting_fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        // let endgame_fen_str: &str = "1k6/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1";
        // let losing_fen_str: &str = "1k6/qq3q2/8/8/8/8/8/4K2Q w KQkq - 0 1";
        // let simple_fen_str: &str = "r3k3/8/8/8/8/8/8/R2QK3 w KQkq - 0 1";
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
            // Selected piece highlighting
            if Some(i) == self.selected_piece_square {
                background_colour = "\x1b[48;5;112m";
            }
            // Possible move highlighting
            if let Some(_possible_move) = self.possible_moves.iter().find(|&m| Some(m.from_square) == self.selected_piece_square &&  m.to_square == i) {
                background_colour = "\x1b[48;5;149m";

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
            possible_moves: vec![],
            selected_piece_square: None,
            colour_in_check: None
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

fn parse_algebraic_move(move_input: &str, game: &Game) -> Option<Move> {
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
        println!("Move is ambiguous, please double disambiguate (e.g. Qh4e1)");
        return None
    }
    println!("Invalid move, use algebraic notation without indication of captures (e.g. Nc3)");
    return None
}

static PAWN_PST: [i32; 64] =
[0,  0,  0,  0,  0,  0,  0,  0,
50, 50, 50, 50, 50, 50, 50, 50,
10, 10, 20, 30, 30, 20, 10, 10,
 5,  5, 10, 25, 25, 10,  5,  5,
 0,  0,  0, 20, 20,  0,  0,  0,
 5, -5,-10,  0,  0,-10, -5,  5,
 5, 10, 10,-20,-20, 10, 10,  5,
 0,  0,  0,  0,  0,  0,  0,  0];

 static KNIGHT_PST: [i32; 64] =
 [-50,-40,-30,-30,-30,-30,-40,-50,
-40,-20,  0,  0,  0,  0,-20,-40,
-30,  0, 10, 15, 15, 10,  0,-30,
-30,  5, 15, 20, 20, 15,  5,-30,
-30,  0, 15, 20, 20, 15,  0,-30,
-30,  5, 10, 15, 15, 10,  5,-30,
-40,-20,  0,  5,  5,  0,-20,-40,
-50,-40,-30,-30,-30,-30,-40,-50];

static BISHOP_PST: [i32; 64] =
[-20,-10,-10,-10,-10,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5, 10, 10,  5,  0,-10,
-10,  5,  5, 10, 10,  5,  5,-10,
-10,  0, 10, 10, 10, 10,  0,-10,
-10, 10, 10, 10, 10, 10, 10,-10,
-10,  5,  0,  0,  0,  0,  5,-10,
-20,-10,-10,-10,-10,-10,-10,-20];

static ROOK_PST: [i32; 64] =
[0,  0,  0,  0,  0,  0,  0,  0,
5, 10, 10, 10, 10, 10, 10,  5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
0,  0,  0,  5,  5,  0,  0,  0];

static QUEEN_PST: [i32; 64] =
[-20,-10,-10, -5, -5,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5,  5,  5,  5,  0,-10,
 -5,  0,  5,  5,  5,  5,  0, -5,
  0,  0,  5,  5,  5,  5,  0, -5,
-10,  5,  5,  5,  5,  5,  0,-10,
-10,  0,  5,  0,  0,  0,  0,-10,
-20,-10,-10, -5, -5,-10,-10,-20];

static KING_MIDDLEGAME: [i32; 64] =
[-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-20,-30,-30,-40,-40,-30,-30,-20,
-10,-20,-20,-20,-20,-20,-20,-10,
 20, 20,  0,  0,  0,  0, 20, 20,
 20, 30, 10,  0,  0, 10, 30, 20];

 // TODO: incorporate this
static KING_ENDGAME: [i32; 64] =
[-50,-40,-30,-20,-20,-30,-40,-50,
-30,-20,-10,  0,  0,-10,-20,-30,
-30,-10, 20, 30, 30, 20,-10,-30,
-30,-10, 30, 40, 40, 30,-10,-30,
-30,-10, 30, 40, 40, 30,-10,-30,
-30,-10, 20, 30, 30, 20,-10,-30,
-30,-30,  0,  0,  0,  0,-30,-30,
-50,-30,-30,-30,-30,-30,-30,-50];



fn evaluate_game(test_game: &mut Game) -> f64 {
    let mut evaluation = 0;
    let mut piece_evaluation = 0;
    for piece in test_game.pieces.clone() {
        let piece_square = bit_to_onebit_index(piece.bit);

        if piece.colour == Colour::White && piece.taken == false {
            match piece.piece_type {
                PieceType::Pawn => piece_evaluation += 100 + PAWN_PST[63 - piece_square],
                PieceType::Bishop => piece_evaluation += 320 + BISHOP_PST[63 - piece_square],
                PieceType::Knight => piece_evaluation += 330 + KNIGHT_PST[63 - piece_square],
                PieceType::Rook => piece_evaluation += 500 + ROOK_PST[63 - piece_square],
                PieceType::Queen => piece_evaluation += 900 + QUEEN_PST[63 - piece_square],
                PieceType::King => piece_evaluation += 20000 + KING_MIDDLEGAME[63 - piece_square],
            }
        }
        if piece.colour == Colour::Black && piece.taken == false {
            match piece.piece_type {
                PieceType::Pawn => piece_evaluation -= 100 + PAWN_PST[piece_square],
                PieceType::Bishop => piece_evaluation -= 320 + BISHOP_PST[piece_square],
                PieceType::Knight => piece_evaluation -= 330 + KNIGHT_PST[piece_square],
                PieceType::Rook => piece_evaluation -= 500 + ROOK_PST[piece_square],
                PieceType::Queen => piece_evaluation -= 900 + QUEEN_PST[piece_square],
                PieceType::King => piece_evaluation -= 20000 + KING_MIDDLEGAME[piece_square],
            }
        }
    }

    evaluation += piece_evaluation;


    return evaluation as f64 / 100.0
}

fn minimax(game: &mut Game, depth: u32, maximizing_player: bool, mut alpha: f64, mut beta: f64) -> (f64, Option<Move>) {
// Base case: If depth is 0 or game over, return the evaluation of the game
    if depth == 0 || game.possible_moves.len() == 0 {
        return (evaluate_game(game), None);
    }

    let mut best_move: Option<Move> = None;
    let mut best_evaluation: f64;

    if maximizing_player {
        best_evaluation = f64::INFINITY;
        for possible_move in &game.possible_moves {
            let mut new_game = game.clone();
            test_move(&mut new_game, *possible_move);
            new_game.possible_moves = generate_moves(&mut new_game);

            let (evaluation, _) = minimax(&mut new_game, depth - 1, false, alpha, beta);

            if evaluation < best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(*possible_move);
            }

            // TODO: Alpha-Beta pruning
        }

    } else {
        best_evaluation = f64::NEG_INFINITY;
        for possible_move in &game.possible_moves {
            let mut new_game = game.clone();
            make_move(&mut new_game, *possible_move);
            new_game.possible_moves = generate_moves(&mut new_game);

            let (evaluation, _) = minimax(&mut new_game, depth - 1, true, alpha, beta);

            if evaluation > best_evaluation {
                best_evaluation = evaluation;
                best_move = Some(*possible_move);
            }

            // TODO: Alpha-Beta pruning
        }
    }

    (best_evaluation, best_move)
}

fn iterative_deepening_minimax(game: &mut Game, max_depth: u32) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut best_evaluation: f64 = f64::NEG_INFINITY;
    
    for depth in 1..=max_depth {
        let (evaluation, best) = minimax(game, depth, true, f64::NEG_INFINITY, f64::INFINITY);
        // Store the best move if evaluation improves
        if evaluation > best_evaluation {
            best_evaluation = evaluation;
            best_move = best;
        }
    }

    best_move
}

pub fn game_loop(mut game: Game) {
    game.possible_moves = generate_moves(&mut game);
    print_board(&game);

    loop {
        if game.pieces.iter().filter(|piece| !piece.taken).count()== 2 {
            println!{"Stalemate!"};
            break
        }
        if game.active_colour == Colour::Black {
            if game.possible_moves.len() == 0 {
                if game.colour_in_check == Some(game.active_colour) {
                    println!{"Checkmate! White wins."};
                } else {
                    println!{"Stalemate!"};
                }
                break
            }
            if game.colour_in_check == Some(game.active_colour) {
                println!("Check!");
            }
            
            println!("Move {:?} ({:?}):", game.fullmove_number, game.active_colour);
            
            println!("Thinking...");
            let max_depth = 2; // You can set the desired depth here
            if let Some(best_move) = iterative_deepening_minimax(&mut game, max_depth) {
                make_move(&mut game, best_move);
                game.possible_moves = generate_moves(&mut game);
                print_board(&game);
            }
        } else {
            if game.possible_moves.len() == 0 {
                if game.colour_in_check == Some(game.active_colour) {
                    println!{"Checkmate! Black wins."};
                } else {
                    println!{"Stalemate!"};
                }
                break
            }
            if game.colour_in_check == Some(game.active_colour) {
                println!("Check!");
            }
            
            println!("Move {:?} ({:?}):", game.fullmove_number, game.active_colour);
            
    

            print!("Enter move: ");
            io::stdout().flush().unwrap();
            let mut move_input = String::new();
            io::stdin().read_line(&mut move_input).unwrap();
            move_input = move_input.trim().to_string();

            if let Some(input_move) = parse_algebraic_move(&move_input, &game) {   
                let start_bit = onebit_index_to_bit(input_move.from_square);
                let start_onebit_index = bit_to_onebit_index(start_bit);
                if let Some(_start_piece_index) = game.pieces.iter().position(|p| p.taken == false && p.bit == start_bit && p.colour == game.active_colour) {
                    game.selected_piece_square = Some(start_onebit_index);
                    print_board(&game);
                    game.selected_piece_square = None;
                    make_move(&mut game, input_move);
                    game.possible_moves = generate_moves(&mut game);
                    print_board(&game);
                } else {
                    print_board(&game);
                    println!("Invalid move");
                }
            }
        }
        
    }
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
            print!("Piece to promote to (Q for Queen, R for Rook, N for Knight, B for Bishop): ");
            io::stdout().flush().unwrap();
            let mut promotion_input = String::new();
            io::stdin().read_line(&mut promotion_input).unwrap();
            promotion_input = promotion_input.trim().to_string();
            let promotion_piece_type = match promotion_input.chars().next().unwrap().to_ascii_lowercase() {
                'q' => PieceType::Queen,
                'r' => PieceType::Rook,
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                _ => {return}
            };
            game.pieces[start_piece_index].piece_type = promotion_piece_type;
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

pub fn print_board(game: &Game) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", game.to_string());
}