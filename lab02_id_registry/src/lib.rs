use std::collections::HashMap;

#[derive(Default)]
pub struct Registry {
    map: HashMap<String, u32>,
    next_id: u32,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_register(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.map.get(name) {
            id
        } else {
            let id = self.next_id;
            self.map.insert(name.to_string(), id);
            self.next_id += 1;
            id
        }
    }
}
