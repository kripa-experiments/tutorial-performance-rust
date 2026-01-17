# Lab 04: Memory Layout with bytemuck

## Tool
`bytemuck`

## Description
This lab defines a struct `ActionData` with `#[repr(C)]`, deriving `Pod` and `Zeroable`.
It implements a function that casts the struct to `&[u8]`.

## Usage
```rust
use lab04_bytemuck_repr::{ActionData, as_bytes};

let action = ActionData {
    id: 1,
    cost: 1.5,
    effect: 100,
};

let bytes = as_bytes(&action);
assert_eq!(bytes.len(), 16);
```
