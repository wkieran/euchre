# euchre

A terminal Euchre game built in Rust with a ratatui TUI.

## Build & Run

```sh
cargo build
cargo run --bin euchre-tui
```

```sh
cargo test
```

## Todo

- [x] Game library — cards, deck, trick, scoring logic
- [x] GameController + tests
- [x] TUI skeleton — nav, polling loop, page routing
- [x] TUI views — dealing, bidding, trick play, scoring, game over
- [ ] TUI/controller integration — event broadcast + human player channel
- [ ] Bot players
- [ ] Networked multiplayer
- [ ] Going alone
