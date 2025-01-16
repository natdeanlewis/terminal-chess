build: `cargo build`
run: `cargo run`
test: `cargo test`
test with println!s :`cargo test -- --nocapture`

- TODOs:
  - turn piece bonuses down or increase check bonus in endgame?
  - use bitboard per piece type?
  - knight done
  - king done
  - rook done
  - bishop done
  - queen done
  - pawn done
  - castling done
  - en passant done
  - pawn promotions done
  - algebraic move notation? done
  - alert when check? done
  - check done
  - checkmate done
  - stalemate done
  - don't allow castling when ONLY king square is threatened (castling out of check) done
  - show last move and eval
  - print lines as calculated
  - tests
  - perft add more layers when faster
  - repetition draws
  - optimisation
  - board evaluation done
  - positional skewing done
  - search done
  - minimax done
  - alphabeta pruning done
  - test alphabeta pruning!
  - play multiple colours
  - mouse gui
  - full algebraic move notation?
  - allow non-queen promotion when testing moves done
  - update and use halfmove and fullmove counts - 50 move rule etc.
  - use bitboards for move calculation
  - use bitboards for threatened square, check calcuation
  - make sure naming is consistent, square, piece index onebit_index, bit etc.

- examples:
  - coords: e4
  - bit: 0000...0000000100000000000 (2^12)
  - onebit_index, square(?) = 12 (0 to 63)
  - piece_index = count of piece (0 to 31)
  - position: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 (board state)
