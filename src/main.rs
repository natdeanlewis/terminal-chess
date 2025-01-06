type PiecePosition = u64;

// fn bit_to_positon(bit: PiecePosition) -> Result<String, String> {
//     if bit == 0 {
//         return Err("No piece present!".to_string());
//     } else {
//         let onebit_index = bit_scan(bit);
//         return Ok(index_to_position(onebit_index));
//     }
// }

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn index_to_position(index: usize) -> String {
    let column = index % 8;
    let row = index / 8 + 1;
    format!("{}{}", COL_MAP[column], row)
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

#[derive(Debug, PartialEq)]
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

#[derive(Debug, PartialEq)]
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
}

impl Game {

    fn push_piece_and_square(&mut self, position: usize, colour: Colour, piece_type: PieceType, index: &mut usize) {
        self.pieces.push(Piece {
            position: 1u64 <<  position,
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
        let mut game = Game { pieces: vec![], squares: vec![] };
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

        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => temp.push_str(&index_to_position(i)),
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
            }

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
        let mut result = match self.piece_type {
            PieceType::Pawn => "p ",
            PieceType::Bishop => "b ",
            PieceType::Knight => "n ",
            PieceType::Rook => "r ",
            PieceType::Queen => "q ",
            PieceType::King => "k ",
        }.to_string();

        if self.colour == Colour::White {
            result.make_ascii_uppercase();
        }

        result
    }
}

fn main() {
    // let pieces = vec![
    //     Piece { position: 1 << 4, colour: Colour::White, piece_type: PieceType::Pawn },
    //     Piece { position: 1 << 20, colour: Colour::Black, piece_type: PieceType::Queen },
    // ];
    // let squares = vec![Square::Empty, Square::Empty, Square::Occupied(1), Square::Occupied(0)];
    // let mut game = Game { pieces: pieces, squares: squares };
    let mut game = Game::initialize();
    println!("{}", game.to_string());
}
