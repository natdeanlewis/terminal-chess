mod utils;
mod game;
mod moves;
mod evaluation;

use game::*;

fn main() {
    let game = Game::initialize();
    game_loop(game)
}
