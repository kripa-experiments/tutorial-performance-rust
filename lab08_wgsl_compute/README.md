# Lab 08: WGSL Logic Gate

## Tool
`wgpu` + WGSL

## Description
This lab implements a compute shader that performs a bitwise AND between two input buffers and writes to an output buffer.

## Usage
```rust
use lab08_wgsl_compute::compute_bitwise_and;

let input1 = vec![0b1010, 0b1111];
let input2 = vec![0b1100, 0b0000];
let result = pollster::block_on(compute_bitwise_and(&input1, &input2));
assert_eq!(result[0], 0b1000);
```
