use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub struct ActionData {
    pub id: u32,
    pub cost: f32,
    pub effect: u64,
}

pub fn as_bytes(action: &ActionData) -> &[u8] {
    bytemuck::bytes_of(action)
}
