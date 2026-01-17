# Lab 06: Data-Oriented Design (Flat Trees)

## Tool
Slices and Indices

## Description
This lab models a task hierarchy (Parent -> Subtasks) using a single flat `Vec<u32>`.
It stores `start_index` and `length` in the `Parent` struct, avoiding recursive pointers.

## Usage
```rust
use lab06_data_oriented_design::TaskManager;

let mut manager = TaskManager::new();

// Parent 0
let p0_subtasks = vec![1, 2, 3];
let p0_idx = manager.add_parent(&p0_subtasks);

let retrieved_p0 = manager.get_subtasks(p0_idx).unwrap();
assert_eq!(retrieved_p0, &[1, 2, 3]);
```
