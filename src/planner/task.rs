#[derive(Clone, Debug)]
pub struct PlanTask {
    pub id: u64,

    // Human-readable task name
    pub name: String,

    // Capability required to execute
    pub capability: String,

    // Input passed to the agent
    pub input: String,

    // Task priority
    pub priority: u8,

    // Parent task (optional)
    pub parent: Option<u64>,

    // IDs of tasks that must finish before this one
    pub dependencies: Vec<u64>,

    // IDs of tasks that depend on this one
    pub children: Vec<u64>,

    // Execution state
    pub completed: bool,

    // Execution result
    pub output: Option<String>,
}

impl PlanTask {
    pub fn new(
        id: u64,
        name: &str,
        capability: &str,
        input: &str,
        priority: u8,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            capability: capability.to_string(),
            input: input.to_string(),
            priority,
            parent: None,
            dependencies: Vec::new(),
            children: Vec::new(),
            completed: false,
            output: None,
        }
    }

    pub fn depends_on(
        &mut self,
        task_id: u64,
    ) {
        if !self.dependencies.contains(&task_id) {
            self.dependencies.push(task_id);
        }
    }

    pub fn add_child(
        &mut self,
        task_id: u64,
    ) {
        if !self.children.contains(&task_id) {
            self.children.push(task_id);
        }
    }

    pub fn set_parent(
        &mut self,
        parent: u64,
    ) {
        self.parent = Some(parent);
    }

    pub fn ready(
        &self,
        completed: &[u64],
    ) -> bool {
        self.dependencies
            .iter()
            .all(|id| completed.contains(id))
    }

    pub fn complete(
        &mut self,
        output: String,
    ) {
        self.completed = true;
        self.output = Some(output);
    }

    pub fn reset(&mut self) {
        self.completed = false;
        self.output = None;
    }

    pub fn is_root(&self) -> bool {
        self.dependencies.is_empty()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}
