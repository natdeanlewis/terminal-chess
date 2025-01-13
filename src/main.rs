mod utils;
mod game;
mod moves;
mod evaluation;
mod knight_moves;
mod pawn_moves;
mod bishop_moves;
mod king_moves;
mod queen_moves;
mod rook_moves;

use game::*;

fn main() {
    let game = Game::initialize();
    game_loop(game)
}
