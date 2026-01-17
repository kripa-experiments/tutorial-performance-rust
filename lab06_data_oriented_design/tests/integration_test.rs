use lab06_data_oriented_design::TaskManager;

#[test]
fn test_flat_tree() {
    let mut manager = TaskManager::new();

    // Parent 0
    let p0_subtasks = vec![1, 2, 3];
    let p0_idx = manager.add_parent(&p0_subtasks);

    // Parent 1
    let p1_subtasks = vec![4, 5];
    let p1_idx = manager.add_parent(&p1_subtasks);

    let retrieved_p0 = manager.get_subtasks(p0_idx).unwrap();
    assert_eq!(retrieved_p0, &[1, 2, 3]);

    let retrieved_p1 = manager.get_subtasks(p1_idx).unwrap();
    assert_eq!(retrieved_p1, &[4, 5]);

    // Verify internal structure logic if needed, but get_subtasks covers it.
    // Check bad index
    assert!(manager.get_subtasks(99).is_none());
}
