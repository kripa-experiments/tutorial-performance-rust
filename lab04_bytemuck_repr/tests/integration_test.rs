use lab04_bytemuck_repr::{ActionData, as_bytes};
use std::mem;

#[test]
fn test_action_data_layout() {
    let action = ActionData {
        id: 1,
        cost: 1.5,
        effect: 100,
    };

    let bytes = as_bytes(&action);

    println!("ActionData size: {}", bytes.len());
    println!("Bytes: {:?}", bytes);

    assert_eq!(bytes.len(), mem::size_of::<ActionData>());

    // Verify values if possible (simple check)
    // id=1 is bytes[0]=1, bytes[1..4]=0 (Little Endian)
    assert_eq!(bytes[0], 1);
}
