mod utils;
mod game;

use game::*;

fn main() {
    let game = Game::initialize();
    print_board(&game);

    game_loop(game)
}
