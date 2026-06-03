# Rust Chess Engine with Terminal UI

A high-performance chess engine written in Rust, featuring an interactive terminal-based interface.

## Features

### Game Modes

* **2-Player** — local human vs. human
* **1-Player Mode** — human vs. engine
* **0-Player Mode** — engine vs. engine demonstration:

![demo.gif](demo.gif)

### Engine Capabilities

* **Bitboard board representation** — 64-bit integers for fast, branch-light move generation
* **Complete legal move generation** — including castling, en passant, pawn promotion, and full check/pin awareness
* **Negamax search with alpha–beta pruning** — prunes irrelevant branches for deeper search in less time
* **Iterative deepening** — progressively deepens the search, stopping early on forced mate
* **Quiescence search** — extends captures past the horizon to avoid tactical blunders
* **Move ordering** — MVV-LVA capture heuristics, promotion bonuses, and pawn-attack penalties to maximise pruning
* **Material + positional evaluation** — piece values combined with piece-square tables for every piece type
* **Phase-aware king evaluation** — separate middlegame and endgame king tables, plus king-distance heuristics to drive mates
* **Draw detection** — threefold repetition and fifty-move rule tracking
* **Full FEN support** — parse and serialise arbitrary positions

---

### Building:

```bash
cargo build --release
```

### Running:

```bash
cargo run --release
```

### Testing:

```bash
cargo test --release
```

Testing with debugging and console output enabled:

```bash
cargo test --release -- --nocapture
```