# Lab 10: GPU Atomic Search Frontier

## Tool
WGSL Atomics

## Description
This lab writes a shader where threads attempt to write their ID to a "frontier" buffer.
It uses `atomicAdd` to manage the write index to avoid race conditions.

## Usage
```rust
use lab10_gpu_atomics::run_atomic_frontier;

let num_threads = 1_000;
let (count, frontier) = pollster::block_on(run_atomic_frontier(num_threads));
assert_eq!(count, 1_000);
```
