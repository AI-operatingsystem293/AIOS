#[derive(Clone, Debug)]
pub struct PlanNode {
    pub id: usize,
    pub name: String,
    pub completed: bool,

    // IDs of prerequisite tasks
    pub depends_on: Vec<usize>,
}

impl PlanNode {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            completed: false,
            depends_on: Vec::new(),
        }
    }

    pub fn depends_on(mut self, task: usize) -> Self {
        self.depends_on.push(task);
        self
    }
}
