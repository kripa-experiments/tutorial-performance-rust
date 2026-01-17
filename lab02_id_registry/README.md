# Lab 02: ID Registry & Interning

## Tool
`std::collections::HashMap`

## Description
This lab implements a `Registry` struct that maps `String` names to `u32` IDs.
It ensures that registering the same string twice returns the same ID.

## Usage
```rust
use lab02_id_registry::Registry;

let mut registry = Registry::new();
let id1 = registry.get_or_register("robot1");
let id2 = registry.get_or_register("roomA");
let id3 = registry.get_or_register("robot1");

assert_eq!(id1, id3);
assert_ne!(id1, id2);
```
