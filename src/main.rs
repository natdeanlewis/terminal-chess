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

use std::io::SeekFrom::Start;
use game::*;
use crate::utils::*;

fn main() {
    let mut game = Game::initialize(_CHECK_OPTIMISATION_FEN_STR);
    game.players = get_players_loop();
    game_loop(game)
}
