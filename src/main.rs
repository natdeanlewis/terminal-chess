mod utils;
mod game;
mod moves;

use game::*;

fn main() {
    let game = Game::initialize();
    game_loop(game)
}
