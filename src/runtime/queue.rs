use crate::task::task::Task;

pub struct RuntimeQueue {
    tasks: Vec<Task>,
}

impl RuntimeQueue {

    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn push(
        &mut self,
        task: Task,
    ) {
        self.tasks.push(task);
    }

    pub fn pop(&mut self) -> Option<Task> {
        if self.tasks.is_empty() {
            None
        } else {
            Some(self.tasks.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}
