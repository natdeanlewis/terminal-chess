mod utils;
mod game;

use game::*;

fn main() {
    let game = Game::initialize();
    game_loop(game)
}
