# Lab 05: Data Parallelism with Rayon

## Tool
`Rayon`

## Description
This lab demonstrates processing a large `Vec<u32>` (1M elements) by performing a complex bit-shift on each element using `Rayon`.
It compares parallel execution with serial execution.

## Usage
```rust
use lab05_rayon_parallelism::process_data_parallel;

let data: Vec<u32> = (0..1_000_000).collect();
let result = process_data_parallel(&data);
assert_eq!(result.len(), 1_000_000);
```
