use lab09_wasm_bindgen::count_set_bits_impl;

#[test]
fn test_popcount() {
    let data = vec![
        0b1010, // 2 bits
        0b1111, // 4 bits
        0b0000, // 0 bits
    ];
    let result = count_set_bits_impl(&data);
    assert_eq!(result, 6);
}
