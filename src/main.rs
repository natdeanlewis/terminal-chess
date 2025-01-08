use std::io;
use std::io::Write;
use bitflags::bitflags;

type PiecePosition = u64;

fn bit_to_position(bit: PiecePosition) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present!".to_string());
    } else {
        let onebit_index = bit_scan(bit);
        return Ok(index_to_position(onebit_index));
    }
}

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn index_to_position(index: usize) -> String {
    let column = index % 8;
    let row = index / 8 + 1;
    format!("{}{}", COL_MAP[column], row)
}

fn coords_to_bit(coords: &str) -> Option<(u8)> {
    let mut chars = coords.chars();
    let column_char = chars.next()?;
    let row_char = chars.next()?;

    let col_index = COL_MAP.iter().position(|&c| c == column_char)? as u8;
    let row = row_char.to_digit(10)? as u8;
    let row_index = row - 1;
    Some(8 * row_index + col_index)
}
fn coords_to_position(coords: &str) -> Option<(u64)>  {
    let bit = coords_to_bit(coords)?;
    let position = 1u64 << bit;
    Some(position)
}

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

fn bit_scan(mut bit: u64) -> usize {
    let remainder: usize = (bit % 67) as usize;
    MOD67TABLE[remainder]
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

struct Game {
    pieces: Vec<Piece>,
    squares: Vec<Square>,
    active_colour: Colour,
    castling_rights: CastlingRights,
    en_passant: Option<PiecePosition>,
    halfmove_clock: usize,
    fullmove_number: usize,
}

bitflags! {
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
    fn push_piece_and_square(&mut self, position: usize, colour: Colour, piece_type: PieceType, index: &mut usize) {
        self.pieces.push(Piece {
            position: 1u64 << position,
            colour: colour,
            piece_type: piece_type,
        });
        self.squares.push(Square::Occupied(*index));
        *index += 1;
    }

    fn push_empty_square(&mut self) {
        self.squares.push(Square::Empty);
    }
    fn initialize() -> Game {
        let mut game = Game { pieces: vec![], squares: vec![] , active_colour: Colour::White, castling_rights: CastlingRights::ALL, en_passant: None, halfmove_clock: 0, fullmove_number: 1};
        let mut piece_index = 0;

        let colour = Colour::White;
        game.push_piece_and_square(0, colour, PieceType::Rook, &mut piece_index);
        game.push_piece_and_square(1, colour, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(2, colour, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(3, colour, PieceType::Queen, &mut piece_index);
        game.push_piece_and_square(4, colour, PieceType::King, &mut piece_index);
        game.push_piece_and_square(5, colour, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(6, colour, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(7, colour, PieceType::Rook, &mut piece_index);
        for i in (8..16) {
            game.push_piece_and_square(i, colour, PieceType::Pawn, &mut piece_index);
        }

        for i in (16..48) {
            game.push_empty_square();
        }

        let colour = Colour::Black;
        for i in (48..56) {
            game.push_piece_and_square(i, colour, PieceType::Pawn, &mut piece_index);
        }
        game.push_piece_and_square(56, colour, PieceType::Rook, &mut piece_index);
        game.push_piece_and_square(57, colour, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(58, colour, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(59, colour, PieceType::Queen, &mut piece_index);
        game.push_piece_and_square(60, colour, PieceType::King, &mut piece_index);
        game.push_piece_and_square(61, colour, PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(62, colour, PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(63, colour, PieceType::Rook, &mut piece_index);

        game
    }

    fn to_string(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();
        board.insert_str(0, "   a  b  c  d  e  f  g  h");
        for (i, square) in self.squares.iter().enumerate() {
            if i % 8 == 0 {
                temp.push_str(((i / 8) + 1).to_string().as_str());
                temp.push_str(" ");
            }

            let background_colour = if i % 2 == (i / 8) % 2 { "\x1b[48;5;130m" } else { "\x1b[48;5;172m" };
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
        Square::Occupied(index) => Some(*index),
        Square::Empty => None,
    }
}

fn main() {
    let mut game = Game::initialize();
    println!("{}", game.to_string());

    loop {
        println!("Move {:?} ({:?}):", game.fullmove_number, game.active_colour);
        print!("Coordinates of piece to move: ");
        io::stdout().flush().unwrap();
        let mut start_input = String::new();
        io::stdin().read_line(&mut start_input).unwrap();

        if let Some(start_position) = coords_to_position(&start_input) {
            if let Some(start_piece_index) = game.pieces.iter().position(|p| p.position == start_position && p.colour == game.active_colour) {
                let mut piece = game.pieces[start_piece_index].clone();

                print!(
                    "Target coordinates for this {:?} {:?}: ",
                    piece.colour, piece.piece_type
                );
                io::stdout().flush().unwrap();
                let mut end_input = String::new();
                io::stdin().read_line(&mut end_input).unwrap();

                if let Some(end_position) = coords_to_position(&end_input) {
                    if let Some(start_square) = coords_to_bit(&start_input) {
                        if let Some(end_square) = coords_to_bit(&end_input) {
                            if let Some(target_index) = game.pieces.iter_mut().position(|p| p.position == end_position) {
                                let target_piece = game.pieces[target_index].clone();
                                if target_piece.colour != piece.colour {
                                    game.squares[end_square as usize] = Square::Empty;
                                    game.pieces[target_index].position = 0;
                                    println!("Captured the {:?} {:?} at {}", target_piece.colour, target_piece.piece_type, end_input);
                                } else {
                                    println!("There is already a {:?} piece at {}", piece.colour, end_input);
                                    continue;
                                }
                            }

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
                            println!("{}", game.to_string());
                        }
                    }
                }
            } else {
                println!("No {:?} piece at {}", game.active_colour, start_input);
            }
        }
    }
}
