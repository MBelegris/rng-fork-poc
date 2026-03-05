# fork-rng-demo

A tiny Rust program that demonstrates a common gotcha when using RNGs across a `fork()`.

On Unix, `fork()` creates a child process by copying the parent’s memory. That means any in-memory RNG state (including thread-local state) can be duplicated. If both parent and child then generate random numbers from what is effectively the same state, they may produce identical sequences.

This example:

- “Warms up” `rand::thread_rng()` in the parent (prints a short pre-fork sequence to stderr).
- Calls `libc::fork()`.
- Re-acquires `rand::thread_rng()` in both parent and child.
- Prints 8 generated `u64`s from each process, showing that the sequences can match due to copied state.
- Waits for the child so output ordering is stable.

### Visualisation

``` rust
parent: thread_rng() -> warmup -> fork()
                 |             |
                 |             +--> child: thread_rng() -> 8 nums -> print -> exit
                 |
                 +----------------> parent: thread_rng() -> 8 nums -> print -> wait
(note: fork copies RNG state, so sequences may match)
```


## Requirements

- Unix-like OS that supports `fork()` (Linux/macOS; **not** Windows).
- Rust toolchain (stable is fine).

## Build & Run

```bash
cargo run --release
```
