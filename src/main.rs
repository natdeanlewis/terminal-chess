mod evaluation;
mod game;
mod moves;
mod moves_bishop;
mod moves_king;
mod moves_knight;
mod moves_pawn;
mod moves_queen;
mod moves_rook;
mod uci;
mod utils;

use crate::utils::*;
use game::*;
use std::io;

fn main() {
    if std::env::args().skip(1).any(|arg| arg == "uci") {
        uci::run(false);
        return;
    }

    eprint!("How many players (0, 1 or 2): ");
    let mut first_line = String::new();
    if io::stdin().read_line(&mut first_line).unwrap_or(0) == 0 {
        return;
    }

    let first_input = first_line.trim();
    if first_input == "uci" {
        uci::run(true);
        return;
    }

    let mut game = Game::initialize(_STARTING_FEN_STR);
    game.players = players_from_first_input(first_input);
    game_loop(game)
}
