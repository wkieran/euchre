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

## Outstanding Work

Items in rough implementation order. Each entry notes where the issue was discovered.

### TUI Integration

1. **Populate `App.view` from events** — `App.on_tick()` drains `event_rx` but never stores a `GameView`. Render functions can't show real game state until this is wired. _Discovered: architecture review, `app.rs:44`_

2. **Wire render functions to `App.view`** — `bidding.rs` and `trickplay.rs` ignore `app` entirely and draw hardcoded stub cards. Once `App.view` is populated these should read from it. _Discovered: architecture review, `tui/bidding.rs:17`, `tui/trickplay.rs:17`_

3. **Wire key input to `action_tx`** — `App.action_tx` is never called. Bidding and trick play pages need to send `GameAction`s back to the controller when the player makes a choice. Currently the human player awaits forever. _Discovered: architecture review, `app.rs:24`_

4. **Add render tests (TestBackend)** — zero tests exist for the TUI binary. Each page module should have a `#[cfg(test)]` block using `Terminal<TestBackend>` with a stub `App`. _Discovered: `cargo test` shows 0 tests in `euchre-tui`_

### Game Logic

5. **`submit_bid` test is incomplete** — `test_submit_bid` in `state.rs` has commented-out assertions and a TODO. The full bidding phase (round 2, stuck dealer, order-up) is untested at the state level. _Discovered: `cargo test`, `state.rs:292`_

6. **`submit_discard` has no test** — called on every ordered-up hand but never directly tested. _Discovered: `state.rs:462` TODO comment_

7. **`current_player` not reset between hands** — `new_deal()` and `start_bidding()` do not reset `current_player`. It carries over from the last trick winner of the previous hand. Likely correct by coincidence (lead goes to last trick winner) but should be explicit. _Discovered: tracing `test_debug_game_runs_to_completion` execution_

### Features

8. **Going alone TUI support** — `is_going_alone` and `going_alone` path exist in game logic but the TUI has no way to bid alone and no visual indication when a player is going alone.

9. **Bot players** — `PassBot` and `DebugBot` are placeholder bots. A real bot needs basic heuristics (order up with a strong hand, lead trump, etc.)

10. **Networked multiplayer** — `PlayerInput` trait is the correct seam; a `NetworkPlayerInput` would implement it. Architecture is ready but not built.
