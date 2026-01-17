# Lab 07: WebGPU Buffer Transfers

## Tool
`wgpu`

## Description
This lab demonstrates how to initialize a `wgpu::Device`, create a buffer, write an array of `u32` to it, and read the data back from the GPU using a mapping/staging buffer.

## Usage
```rust
use lab07_wgpu_basics::run_buffer_transfer;

let input_data = vec![1, 2, 3, 4, 100, 200];
let output_data = pollster::block_on(run_buffer_transfer(&input_data));
assert_eq!(input_data, output_data);
```
