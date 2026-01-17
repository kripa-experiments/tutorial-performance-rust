# Lab 03: Bit-Vector World State

## Tool
Raw bitwise operators on `[u64; 2]`

## Description
This lab implements a `WorldState` that tracks boolean facts via bit indices.
It demonstrates efficient bit manipulation for managing world state.

## Usage
```rust
use lab03_bit_manipulation::WorldState;

let mut state = WorldState::new();
state.set(10);
state.set(65);

let mut mask = WorldState::new();
mask.set(10);
mask.set(65);

assert!(state.check_precondition(&mask));
```
