# Lab 09: WASM Linear Memory

## Tool
`wasm-bindgen`

## Description
This lab exports a Rust function that takes a `js_sys::Uint32Array` from JS and returns the count of set bits (popcount).
It demonstrates direct typed array access.

## Usage
(In JavaScript)
```javascript
import { count_set_bits } from "lab09_wasm_bindgen";

const array = new Uint32Array([10, 15, 0]);
const count = count_set_bits(array);
console.log(count); // 6
```
