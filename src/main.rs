mod evaluation;
mod game;
mod moves;
mod moves_bishop;
mod moves_king;
mod moves_knight;
mod moves_pawn;
mod moves_queen;
mod moves_rook;
mod utils;

use crate::utils::*;
use game::*;

fn main() {
    let mut game = Game::initialize(_STARTING_FEN_STR);
    game.players = get_players_loop();
    game_loop(game)
}
