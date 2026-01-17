use lab05_rayon_parallelism::{process_data_parallel, process_data_serial};

#[test]
fn test_parallel_vs_serial() {
    let data: Vec<u32> = (0..1_000_000).collect();

    let parallel_result = process_data_parallel(&data);
    let serial_result = process_data_serial(&data);

    assert_eq!(parallel_result, serial_result);
    assert_eq!(parallel_result.len(), 1_000_000);

    // Check a value to make sure it did something
    let first = parallel_result[1]; // x = 1
    // 1 << 13 = 8192. 1 ^ 8192 = 8193.
    // 8193 >> 17 = 0. 8193 ^ 0 = 8193.
    // 8193 << 5 = 262176. 8193 ^ 262176 = 270369.
    assert_eq!(first, 270369);
}
