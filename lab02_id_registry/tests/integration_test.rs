use lab02_id_registry::Registry;

#[test]
fn test_registry() {
    let mut registry = Registry::new();
    let id1 = registry.get_or_register("robot1");
    let id2 = registry.get_or_register("roomA");
    let id3 = registry.get_or_register("robot1");

    println!("robot1 id: {}", id1);
    println!("roomA id: {}", id2);
    println!("robot1 id (second time): {}", id3);

    assert_eq!(id1, id3);
    assert_ne!(id1, id2);
}
