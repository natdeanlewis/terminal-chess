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
use evaluation::iterative_deepening_minimax;
use game::*;
use moves::make_move;
use std::io::{self, Write};

fn main() {
    loop {
        io::stdout().flush().unwrap();
        let mut gui_message = String::new();
        io::stdin().read_line(&mut gui_message).unwrap();
        let gui_message = gui_message.trim().to_string();
        if gui_message == "uci" {
            io::stdout().write_all(b"id name Chessbot\n");
            io::stdout().write_all(b"uciok\n");
        } else if gui_message == "isready" {
            io::stdout().write_all(b"readyok\n");

            let mut game = Game::initialize(_STARTING_FEN_STR);
            loop {
                io::stdout().flush().unwrap();
                let mut gui_message = String::new();
                io::stdin().read_line(&mut gui_message).unwrap();
                let gui_message = gui_message.trim().to_string();

                let (first, rest) = split_on(&gui_message, ' ');

                if first == "position" {
                    // Handle "position startpos" or "position fen <FEN>"
                    game = if rest.starts_with("startpos") {
                        Game::initialize(_STARTING_FEN_STR)
                    } else if rest.starts_with("fen") {
                        let fen = rest
                            .trim_start_matches("fen ")
                            .split(" moves ")
                            .next()
                            .unwrap_or("")
                            .to_string();
                        Game::initialize(&fen)
                    } else {
                        return; // Invalid input, exit early
                    };

                    // Extract moves if present
                    let moves = rest.split(" moves ").nth(1).unwrap_or_default();
                    let played_moves: Vec<&str> = moves.split_whitespace().collect();

                    for played_move in played_moves {
                        let move_to_make = uci_notation_to_move(played_move.to_owned());
                        let move_to_unmake = make_move(&mut game, move_to_make);
                        // position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves g1f3 g8f6
                    }
                } else if first == "go" {
                    let first = split_on(&gui_message, ' ').0;
                    io::stdout()
                        .write_all(format!("info depth 1 seldepth 0\n").as_bytes())
                        .expect("Failed to write");
                } else if first == "stop" {
                    if let Some(best_move) = iterative_deepening_minimax(&mut game, 2) {
                        let notated_move = move_to_uci_notation(best_move);
                        io::stdout()
                            .write_all(format!("bestmove {}\n", notated_move).as_bytes())
                            .expect("Failed to write");
                        break;
                    }
                }
            }
        }
    }

    // game.players = get_players_loop();
    // game_loop(game)
}
