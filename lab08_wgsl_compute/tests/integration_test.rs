use lab08_wgsl_compute::compute_bitwise_and;

#[test]
fn test_compute_shader() {
    let input1 = vec![0b1010, 0b1111];
    let input2 = vec![0b1100, 0b0000];

    let result = pollster::block_on(compute_bitwise_and(&input1, &input2));

    println!("Result: {:?}", result);

    assert_eq!(result[0], 0b1000);
    assert_eq!(result[1], 0b0000);
}
