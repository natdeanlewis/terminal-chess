# Rust Chess Engine with Terminal UI

A high-performance chess engine written in Rust, featuring an interactive terminal-based interface.

## Features

### Game Modes

* **2-Player** — local human vs. human
* **1-Player Mode** — human vs. engine
* **0-Player Mode** — engine vs. engine demonstration:

![demo.gif](demo.gif)

### Engine Capabilities

* Complete legal move generation
* Position evaluation and move selection
* Automated opponent play
* Autonomous self-play for testing and analysis



## Technology Stack

* **Language:** Rust
* **Interface:** Terminal / CLI
* **Build System:** Cargo

---

## Building

Compile an optimized release build:

```bash
cargo build --release
```

---

## Running

Launch the application:

```bash
cargo run --release
```

---

## Testing

Run the full test suite:

```bash
cargo test --release
```

Run tests with debugging and console output enabled:

```bash
cargo test --release -- --nocapture
```