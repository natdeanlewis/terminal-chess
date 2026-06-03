# Rust Chess Engine with Terminal UI

A high-performance chess engine written in **Rust**, featuring an interactive terminal-based interface.

Three game modes:
2-player, 1-player (user vs computer) or 0-player (engine plays itself):

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