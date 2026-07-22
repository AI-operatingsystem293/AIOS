pub struct MasterContext {
    pub goal: String,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
}

impl MasterContext {
    pub fn new(goal: &str) -> Self {
        Self {
            goal: goal.to_string(),
            completed_tasks: 0,
            failed_tasks: 0,
        }
    }
}
