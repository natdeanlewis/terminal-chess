use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::sync::OnceLock;

use crate::evaluation::search_to_depth;
use crate::game::Game;
use crate::moves::make_move;
use crate::utils::{move_to_uci_notation, split_on, uci_notation_to_move, _STARTING_FEN_STR};

const ENGINE_NAME: &str = concat!("RustChess ", env!("CARGO_PKG_VERSION"));
const ENGINE_AUTHOR: &str = "Nat Dean-Lewis";

const DEFAULT_DEPTH: u32 = 5;

pub fn run(handshake_received: bool) {
    if handshake_received {
        identify();
    }

    let mut game = Game::initialize(_STARTING_FEN_STR);
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        let line = line.trim();
        log_io('>', line);
        let (command, rest) = split_on(line, ' ');

        match command {
            "uci" => identify(),
            "isready" => respond("readyok"),
            "ucinewgame" => game = Game::initialize(_STARTING_FEN_STR),
            "position" => game = parse_position(rest),
            "go" => {
                let depth = parse_go_depth(rest).unwrap_or(DEFAULT_DEPTH);
                go(&mut game, depth);
            }
            "stop" => {}
            "quit" => break,
            _ => {}
        }
    }
}

fn go(game: &mut Game, max_depth: u32) {
    let mut best_move = None;

    for depth in 1..=max_depth {
        let (score, found) = search_to_depth(game, depth);
        if let Some(found_move) = found {
            best_move = Some(found_move);
            let centipawns = (score * 100.0).round() as i64;
            respond(&format!(
                "info depth {} score cp {} pv {}",
                depth,
                centipawns,
                move_to_uci_notation(found_move)
            ));
        }
    }

    match best_move {
        Some(best) => respond(&format!("bestmove {}", move_to_uci_notation(best))),
        None => respond("bestmove 0000"),
    }
}

fn identify() {
    respond(&format!("id name {}", ENGINE_NAME));
    respond(&format!("id author {}", ENGINE_AUTHOR));
    respond("uciok");
}

fn parse_position(rest: &str) -> Game {
    let rest = rest.trim();

    let (spec, moves) = match rest.find(" moves ") {
        Some(index) => (rest[..index].trim(), rest[index + " moves ".len()..].trim()),
        None => (rest, ""),
    };

    let mut game = if let Some(fen) = spec.strip_prefix("fen ") {
        Game::initialize(fen.trim())
    } else {
        Game::initialize(_STARTING_FEN_STR)
    };

    let fen_string = Game::write_FEN_without_move_counts(&game);
    *game.position_counts.entry(fen_string).or_insert(0) += 1;

    for played_move in moves.split_whitespace() {
        match uci_notation_to_move(played_move) {
            Ok(parsed_move) => {
                make_move(&mut game, parsed_move);
                game.last_move = Some(parsed_move);
            }
            Err(message) => eprintln!("info string skipping move '{}': {}", played_move, message),
        }
    }

    game
}

fn parse_go_depth(rest: &str) -> Option<u32> {
    let mut tokens = rest.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "depth" {
            return tokens.next()?.parse().ok();
        }
    }
    None
}

fn respond(message: &str) {
    log_io('<', message);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let _ = writeln!(handle, "{}", message);
    let _ = handle.flush();
}

fn log_io(direction: char, message: &str) {
    static LOG_PATH: OnceLock<Option<String>> = OnceLock::new();
    let path = LOG_PATH.get_or_init(|| std::env::var("CHESS_UCI_LOG").ok());
    if let Some(path) = path {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(file, "{} {}", direction, message);
        }
    }
}
