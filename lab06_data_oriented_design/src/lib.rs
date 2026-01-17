pub struct Parent {
    pub subtask_start_index: u32,
    pub subtask_length: u32,
}

pub struct TaskManager {
    pub parents: Vec<Parent>,
    pub all_tasks: Vec<u32>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            parents: Vec::new(),
            all_tasks: Vec::new(),
        }
    }

    pub fn add_parent(&mut self, subtasks: &[u32]) -> usize {
        let start_index = self.all_tasks.len() as u32;
        self.all_tasks.extend_from_slice(subtasks);
        let length = subtasks.len() as u32;

        self.parents.push(Parent {
            subtask_start_index: start_index,
            subtask_length: length,
        });

        self.parents.len() - 1
    }

    pub fn get_subtasks(&self, parent_index: usize) -> Option<&[u32]> {
        self.parents.get(parent_index).map(|parent| {
            let start = parent.subtask_start_index as usize;
            let end = start + parent.subtask_length as usize;
            &self.all_tasks[start..end]
        })
    }
}
