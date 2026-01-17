use lab10_gpu_atomics::run_atomic_frontier;
use std::collections::HashSet;

#[test]
fn test_atomic_frontier() {
    let num_threads = 1_000;
    let (count, frontier) = pollster::block_on(run_atomic_frontier(num_threads));

    assert_eq!(count, num_threads);

    let mut ids = HashSet::new();
    for i in 0..num_threads as usize {
        let id = frontier[i];
        ids.insert(id);
    }

    assert_eq!(ids.len(), num_threads as usize);
    for i in 0..num_threads {
        assert!(ids.contains(&i));
    }
}
