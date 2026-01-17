use lab07_wgpu_basics::run_buffer_transfer;

#[test]
fn test_buffer_transfer() {
    let input_data = vec![1, 2, 3, 4, 100, 200];
    let output_data = pollster::block_on(run_buffer_transfer(&input_data));

    assert_eq!(input_data, output_data);
}
