#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WorldState {
    pub data: [u64; 2],
}

impl WorldState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, bit: u32) {
        if bit < 64 {
            self.data[0] |= 1 << bit;
        } else if bit < 128 {
            self.data[1] |= 1 << (bit - 64);
        }
    }

    pub fn check_precondition(&self, mask: &WorldState) -> bool {
        (self.data[0] & mask.data[0] == mask.data[0]) &&
        (self.data[1] & mask.data[1] == mask.data[1])
    }

    pub fn apply_effect(&self, add: &WorldState, del: &WorldState) -> WorldState {
        let mut new_state = *self;
        new_state.data[0] &= !del.data[0];
        new_state.data[0] |= add.data[0];
        new_state.data[1] &= !del.data[1];
        new_state.data[1] |= add.data[1];
        new_state
    }
}
