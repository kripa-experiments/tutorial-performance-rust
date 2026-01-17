use lab03_bit_manipulation::WorldState;

#[test]
fn test_world_state() {
    let mut state = WorldState::new();
    state.set(10);
    state.set(65);

    let mut mask = WorldState::new();
    mask.set(10);
    mask.set(65);

    assert!(state.check_precondition(&mask));

    let mut bad_mask = WorldState::new();
    bad_mask.set(10);
    bad_mask.set(66);

    assert!(!state.check_precondition(&bad_mask));
}

#[test]
fn test_apply_effect() {
    let mut state = WorldState::new();
    state.set(10);
    state.set(65);

    let mut add = WorldState::new();
    add.set(20);
    add.set(70);

    let mut del = WorldState::new();
    del.set(10);
    del.set(65);

    let new_state = state.apply_effect(&add, &del);

    let mut expected = WorldState::new();
    expected.set(20);
    expected.set(70);

    assert_eq!(new_state, expected);
}
