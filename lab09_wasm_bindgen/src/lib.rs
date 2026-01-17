use wasm_bindgen::prelude::*;

pub fn count_set_bits_impl(data: &[u32]) -> u32 {
    data.iter().map(|&x| x.count_ones()).sum()
}

#[wasm_bindgen]
pub fn count_set_bits(array: &js_sys::Uint32Array) -> u32 {
    let vec = array.to_vec();
    count_set_bits_impl(&vec)
}
