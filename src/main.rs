mod utils;
mod game;
mod moves;
mod evaluation;
mod moves_knight;
mod moves_pawn;
mod moves_bishop;
mod moves_king;
mod moves_queen;
mod moves_rook;

use game::*;

fn main() {
    let game = Game::initialize();
    game_loop(game)
}
